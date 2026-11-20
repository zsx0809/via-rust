#!/usr/bin/env python3
import json
import os
import re
import subprocess
import sys
import time
import urllib.request

TOKEN_PATH = "/data/data/com.termux/files/home/projects/antigravity-test/服务器访问凭据（勿入库）.md"

def get_token():
    with open(TOKEN_PATH, "r", encoding="utf-8") as f:
        text = f.read()
    match = re.search(r"ghp_[A-Za-z0-9_]+", text)
    if not match:
        raise RuntimeError("GitHub Token not found")
    return match.group(0)

def github_api_request(url, token):
    req = urllib.request.Request(url)
    req.add_header("Authorization", f"Bearer {token}")
    req.add_header("Accept", "application/vnd.github+json")
    req.add_header("User-Agent", "ViaRust-CI-Monitor")
    try:
        with urllib.request.urlopen(req, timeout=15) as resp:
            return json.loads(resp.read().decode("utf-8"))
    except Exception as e:
        print(f"[Monitor] API Error: {e}", flush=True)
        return None

def main():
    token = get_token()
    repo = "zsx0809/via-rust"
    runs_url = f"https://api.github.com/repos/{repo}/actions/runs?per_page=5"

    print("[Monitor] Fetching latest workflow run...", flush=True)
    time.sleep(3)
    
    target_run = None
    for _ in range(10):
        data = github_api_request(runs_url, token)
        if data and "workflow_runs" in data and len(data["workflow_runs"]) > 0:
            target_run = data["workflow_runs"][0]
            print(f"[Monitor] Tracking Run #{target_run['run_number']} (ID: {target_run['id']}) - Status: {target_run['status']}", flush=True)
            break
        time.sleep(3)

    if not target_run:
        print("[Monitor] No run found, exiting.", flush=True)
        return 1

    run_id = target_run["id"]
    run_url = f"https://api.github.com/repos/{repo}/actions/runs/{run_id}"

    # 轮询状态
    start_time = time.time()
    while True:
        data = github_api_request(run_url, token)
        if data:
            status = data.get("status")
            conclusion = data.get("conclusion")
            elapsed = int(time.time() - start_time)
            print(f"[{elapsed}s] Run #{data.get('run_number')} Status: {status}, Conclusion: {conclusion}", flush=True)
            if status == "completed":
                if conclusion == "success":
                    print("[Monitor] Build successful! Fetching release asset...", flush=True)
                    break
                else:
                    print(f"[Monitor] Build failed with conclusion: {conclusion}", flush=True)
                    return 1
        time.sleep(10)

    # 从 Release 中获取 APK
    release_url = f"https://api.github.com/repos/{repo}/releases/tags/v0.1.0-alpha"
    release_data = github_api_request(release_url, token)
    apk_download_url = None
    if release_data and "assets" in release_data:
        for asset in release_data["assets"]:
            if asset["name"].endswith(".apk"):
                apk_download_url = asset["browser_download_url"]
                print(f"[Monitor] Found APK download URL: {apk_download_url}", flush=True)
                break

    apk_path = "/data/local/tmp/app.apk"
    if apk_download_url:
        print(f"[Monitor] Downloading APK to {apk_path}...", flush=True)
        # 用 curl 带授权下载到 /data/local/tmp/app.apk
        cmd = f'su -c "curl -L -H \\"Authorization: Bearer {token}\\" \\"{apk_download_url}\\" -o {apk_path} && chmod 644 {apk_path}"'
        subprocess.run(cmd, shell=True, check=True)
    else:
        print("[Monitor] No release asset found directly, checking artifacts...", flush=True)

    # 执行安装
    print("[Monitor] Installing APK via root pm...", flush=True)
    install_res = subprocess.run(f'su -c "pm install -r {apk_path}"', shell=True, capture_output=True, text=True)
    print(f"[Monitor] Install output: {install_res.stdout.strip()} {install_res.stderr.strip()}", flush=True)

    # 启动应用
    print("[Monitor] Starting Via application...", flush=True)
    subprocess.run('su -c "am start -n com.zsx0809.viarust/android.app.NativeActivity"', shell=True)
    time.sleep(2)

    # 验证是否存活
    ps_res = subprocess.run('su -c "pidof com.zsx0809.viarust"', shell=True, capture_output=True, text=True)
    pid = ps_res.stdout.strip()
    if pid:
        print(f"[Monitor] SUCCESS! Via is running with PID: {pid}", flush=True)
    else:
        print("[Monitor] App not found in process table, checking crash log...", flush=True)
        crash = subprocess.run('su -c "logcat -b crash -d | tail -n 25"', shell=True, capture_output=True, text=True)
        print(crash.stdout)

    return 0

if __name__ == "__main__":
    sys.exit(main())
