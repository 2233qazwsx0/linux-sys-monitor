#include "install.h"
#include "window.h"
#include <shlobj.h>
#include <shlguid.h>
#include <strsafe.h>
#include <algorithm>
#include <wininet.h>

#pragma comment(lib, "shell32.lib")
#pragma comment(lib, "ole32.lib")
#pragma comment(lib, "shfolder.lib")
#pragma comment(lib, "wininet.lib")

static const wchar_t REPO_URL[] = L"https://github.com/2233qazwsx0/linux-sys-monitor.git";
static const wchar_t EXE_NAME[] = L"linux-sys-monitor.exe";

static std::wstring GetTempPath()
{
    wchar_t tempPath[MAX_PATH];
    GetTempPathW(MAX_PATH, tempPath);
    std::wstring result = tempPath;
    if (!result.empty() && result.back() != L'\\')
        result += L'\\';
    result += L"linux_sys_monitor_install\\";
    CreateDirectoryW(result.c_str(), NULL);
    return result;
}

bool CheckRustInstalled(wchar_t* versionBuffer, int bufferSize)
{
    if (!versionBuffer || bufferSize <= 0)
        return false;

    versionBuffer[0] = L'\0';

    HKEY hKey;
    LONG result = RegOpenKeyExW(HKEY_LOCAL_MACHINE,
        L"SOFTWARE\\Rust\\Rustup", 0, KEY_READ, &hKey);
    if (result != ERROR_SUCCESS)
    {
        result = RegOpenKeyExW(HKEY_CURRENT_USER,
            L"SOFTWARE\\Rust\\Rustup", 0, KEY_READ, &hKey);
    }
    if (result != ERROR_SUCCESS)
    {
        wchar_t rustcPath[MAX_PATH];
        if (GetEnvironmentVariableW(L"RUSTUP_HOME", rustcPath, MAX_PATH))
        {
            wcscat_s(rustcPath, MAX_PATH, L"\\bin\\rustc.exe");
            if (GetFileAttributesW(rustcPath) != INVALID_FILE_ATTRIBUTES)
            {
                wcscpy_s(versionBuffer, bufferSize, L"rustup");
                return true;
            }
        }

        if (GetEnvironmentVariableW(L"CARGO_HOME", rustcPath, MAX_PATH))
        {
            wcscat_s(rustcPath, MAX_PATH, L"\\bin\\rustc.exe");
            if (GetFileAttributesW(rustcPath) != INVALID_FILE_ATTRIBUTES)
            {
                wcscpy_s(versionBuffer, bufferSize, L"cargo");
                return true;
            }
        }
        return false;
    }

    DWORD type;
    DWORD cbData = bufferSize * sizeof(wchar_t);
    result = RegQueryValueExW(hKey, L"DefaultToolchain", NULL, &type,
        (LPBYTE)versionBuffer, &cbData);
    RegCloseKey(hKey);

    if (result == ERROR_SUCCESS && type == REG_SZ)
        return true;

    wcscpy_s(versionBuffer, bufferSize, L"installed");
    return true;
}

bool CheckGitInstalled(wchar_t* versionBuffer, int bufferSize)
{
    if (!versionBuffer || bufferSize <= 0)
        return false;

    versionBuffer[0] = L'\0';

    wchar_t gitPath[MAX_PATH];
    DWORD size = sizeof(gitPath);

    HKEY hKey;
    if (RegOpenKeyExW(HKEY_LOCAL_MACHINE,
        L"SOFTWARE\\GitForWindows", 0, KEY_READ, &hKey) == ERROR_SUCCESS)
    {
        size = sizeof(gitPath);
        if (RegQueryValueExW(hKey, L"InstallPath", NULL, NULL,
            (LPBYTE)gitPath, &size) == ERROR_SUCCESS)
        {
            RegCloseKey(hKey);
            wcscat_s(gitPath, MAX_PATH, L"\\bin\\git.exe");
            if (GetFileAttributesW(gitPath) != INVALID_FILE_ATTRIBUTES)
            {
                wcscpy_s(versionBuffer, bufferSize, L"Git for Windows");
                return true;
            }
        }
        RegCloseKey(hKey);
    }

    if (RegOpenKeyExW(HKEY_LOCAL_MACHINE,
        L"SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Git_is1",
        0, KEY_READ, &hKey) == ERROR_SUCCESS)
    {
        size = bufferSize * sizeof(wchar_t);
        if (RegQueryValueExW(hKey, L"DisplayVersion", NULL, NULL,
            (LPBYTE)versionBuffer, &size) == ERROR_SUCCESS)
        {
            RegCloseKey(hKey);
            return true;
        }
        RegCloseKey(hKey);
    }

    if (GetEnvironmentVariableW(L"GIT_INSTALL_ROOT", gitPath, MAX_PATH))
    {
        wcscat_s(gitPath, MAX_PATH, L"\\bin\\git.exe");
        if (GetFileAttributesW(gitPath) != INVALID_FILE_ATTRIBUTES)
        {
            wcscpy_s(versionBuffer, bufferSize, L"Git");
            return true;
        }
    }

    if (SearchPathW(NULL, L"git.exe", NULL, MAX_PATH, gitPath, NULL) > 0)
    {
        wcscpy_s(versionBuffer, bufferSize, L"Git");
        return true;
    }

    return false;
}

bool InstallRust()
{
    std::wstring rustupPath = GetTempPath() + L"rustup-init.exe";

    HINTERNET hInternet = InternetOpenW(L"Rustup Installer", INTERNET_OPEN_TYPE_PRECONFIG, NULL, NULL, 0);
    if (hInternet)
    {
        HINTERNET hFile = InternetOpenUrlW(hInternet,
            L"https://win.rustup.rs",
            NULL, 0,
            INTERNET_FLAG_NO_CACHE_WRITE | INTERNET_FLAG_PRAGMA_NOCACHE | INTERNET_FLAG_RELOAD,
            0);

        if (hFile)
        {
            HANDLE hOutFile = CreateFileW(rustupPath.c_str(), GENERIC_WRITE, 0, NULL,
                CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, NULL);

            if (hOutFile != INVALID_HANDLE_VALUE)
            {
                BYTE buffer[8192];
                DWORD bytesRead, bytesWritten;

                while (InternetReadFile(hFile, buffer, sizeof(buffer), &bytesRead) && bytesRead > 0)
                {
                    WriteFile(hOutFile, buffer, bytesRead, &bytesWritten, NULL);
                }

                CloseHandle(hOutFile);
            }

            InternetCloseHandle(hFile);
        }

        InternetCloseHandle(hInternet);
    }

    if (GetFileAttributesW(rustupPath.c_str()) != INVALID_FILE_ATTRIBUTES)
    {
        ShellExecuteW(NULL, L"open", rustupPath.c_str(),
            L"-y --default-toolchain stable", NULL, SW_SHOWNORMAL);
        return true;
    }

    ShellExecuteW(NULL, L"open", L"https://win.rustup.rs", NULL, NULL, SW_SHOWNORMAL);
    return false;
}

bool InstallGit()
{
    ShellExecuteW(NULL, L"open", L"https://git-scm.com/download/win", NULL, NULL, SW_SHOWNORMAL);
    return false;
}

bool CloneRepository(const wchar_t* destPath, InstallerWindow* pWindow)
{
    if (!destPath || !pWindow)
        return false;

    std::wstring cmd = L"git clone --depth 1 ";
    cmd += REPO_URL;
    cmd += L" \"";
    cmd += destPath;
    cmd += L"\"";

    if (pWindow)
        pWindow->UpdateLog(L"正在克隆代码仓库...");

    SECURITY_ATTRIBUTES sa;
    sa.nLength = sizeof(sa);
    sa.lpSecurityDescriptor = NULL;
    sa.bInheritHandle = TRUE;

    HANDLE hReadPipe, hWritePipe;
    if (!CreatePipe(&hReadPipe, &hWritePipe, &sa, 0))
        return false;

    STARTUPINFOW si;
    memset(&si, 0, sizeof(si));
    si.cb = sizeof(si);
    si.dwFlags = STARTF_USESTDHANDLES | STARTF_USESHOWWINDOW;
    si.hStdOutput = hWritePipe;
    si.hStdError = hWritePipe;
    si.wShowWindow = SW_HIDE;

    PROCESS_INFORMATION pi;
    memset(&pi, 0, sizeof(pi));

    BOOL bSuccess = CreateProcessW(NULL, &cmd[0], NULL, NULL, TRUE,
        CREATE_NO_WINDOW, NULL, NULL, &si, &pi);

    if (!bSuccess)
    {
        CloseHandle(hReadPipe);
        CloseHandle(hWritePipe);
        return false;
    }

    CloseHandle(hWritePipe);

    char buffer[4096];
    DWORD bytesRead;
    std::string output;

    while (ReadFile(hReadPipe, buffer, sizeof(buffer) - 1, &bytesRead, NULL) && bytesRead > 0)
    {
        buffer[bytesRead] = '\0';
        output += buffer;

        if (pWindow)
        {
            wchar_t wBuffer[4096];
            MultiByteToWideChar(CP_UTF8, 0, buffer, bytesRead, wBuffer, 4095);
            wBuffer[bytesRead < 4095 ? bytesRead : 4095] = L'\0';

            wchar_t* line = wcstok(wBuffer, L"\r\n");
            while (line)
            {
                pWindow->UpdateLog(line);
                line = wcstok(NULL, L"\r\n");
            }
        }
    }

    CloseHandle(hReadPipe);

    WaitForSingleObject(pi.hProcess, INFINITE);

    DWORD exitCode;
    GetExitCodeProcess(pi.hProcess, &exitCode);

    CloseHandle(pi.hProcess);
    CloseHandle(pi.hThread);

    if (exitCode != 0)
    {
        if (pWindow)
            pWindow->UpdateLog(L"错误: 仓库克隆失败");
        return false;
    }

    if (pWindow)
        pWindow->UpdateLog(L"代码仓库克隆完成");

    return true;
}

bool BuildProject(const wchar_t* repoPath, InstallerWindow* pWindow)
{
    if (!repoPath || !pWindow)
        return false;

    if (pWindow)
    {
        pWindow->UpdateLog(L"正在编译项目...");
        pWindow->SetProgress(50);
    }

    std::wstring workDir = repoPath;
    if (!workDir.empty() && workDir.back() != L'\\')
        workDir += L'\\';

    std::wstring cmd = L"cargo build --release";

    SECURITY_ATTRIBUTES sa;
    sa.nLength = sizeof(sa);
    sa.lpSecurityDescriptor = NULL;
    sa.bInheritHandle = TRUE;

    HANDLE hReadPipe, hWritePipe;
    if (!CreatePipe(&hReadPipe, &hWritePipe, &sa, 0))
        return false;

    STARTUPINFOW si;
    memset(&si, 0, sizeof(si));
    si.cb = sizeof(si);
    si.dwFlags = STARTF_USESTDHANDLES | STARTF_USESHOWWINDOW;
    si.hStdOutput = hWritePipe;
    si.hStdError = hWritePipe;
    si.wShowWindow = SW_HIDE;

    PROCESS_INFORMATION pi;
    memset(&pi, 0, sizeof(pi));

    BOOL bSuccess = CreateProcessW(NULL, &cmd[0], NULL, NULL, TRUE,
        CREATE_NO_WINDOW, NULL, workDir.c_str(), &si, &pi);

    if (!bSuccess)
    {
        CloseHandle(hReadPipe);
        CloseHandle(hWritePipe);
        if (pWindow)
            pWindow->UpdateLog(L"错误: 无法启动 cargo 编译");
        return false;
    }

    CloseHandle(hWritePipe);

    char buffer[4096];
    DWORD bytesRead;
    std::string output;
    int lastProgress = 50;

    while (ReadFile(hReadPipe, buffer, sizeof(buffer) - 1, &bytesRead, NULL) && bytesRead > 0)
    {
        buffer[bytesRead] = '\0';
        output += buffer;

        if (pWindow)
        {
            wchar_t wBuffer[4096];
            int wLen = MultiByteToWideChar(CP_UTF8, 0, buffer, bytesRead, wBuffer, 4095);
            wBuffer[wLen] = L'\0';

            wchar_t* line = wcstok(wBuffer, L"\r\n");
            while (line)
            {
                pWindow->UpdateLog(line);
                line = wcstok(NULL, L"\r\n");
            }

            std::string lowerOutput = output;
            std::transform(lowerOutput.begin(), lowerOutput.end(), lowerOutput.begin(), ::tolower);
            if (lowerOutput.find("compiling") != std::string::npos ||
                lowerOutput.find("building") != std::string::npos)
            {
                if (pWindow->GetCurrentPage() == PAGE_INSTALLING)
                {
                    int newProgress = 50 + (int)(output.size() / 10) % 40;
                    if (newProgress > 90) newProgress = 90;
                    if (newProgress != lastProgress)
                    {
                        pWindow->SetProgress(newProgress);
                        lastProgress = newProgress;
                    }
                }
            }
        }
    }

    CloseHandle(hReadPipe);

    WaitForSingleObject(pi.hProcess, INFINITE);

    DWORD exitCode;
    GetExitCodeProcess(pi.hProcess, &exitCode);

    CloseHandle(pi.hProcess);
    CloseHandle(pi.hThread);

    if (exitCode != 0)
    {
        if (pWindow)
            pWindow->UpdateLog(L"错误: 编译失败");
        return false;
    }

    if (pWindow)
    {
        pWindow->UpdateLog(L"编译完成");
        pWindow->SetProgress(95);
    }

    return true;
}

bool CopyBinary(const wchar_t* repoPath, const wchar_t* installPath, InstallerWindow* pWindow)
{
    if (!repoPath || !installPath || !pWindow)
        return false;

    if (pWindow)
        pWindow->UpdateLog(L"正在复制文件...");

    std::wstring srcBin = repoPath;
    if (!srcBin.empty() && srcBin.back() != L'\\')
        srcBin += L'\\';
    srcBin += L"target\\release\\";
    srcBin += EXE_NAME;

    if (GetFileAttributesW(srcBin.c_str()) == INVALID_FILE_ATTRIBUTES)
    {
        srcBin = repoPath;
        if (!srcBin.empty() && srcBin.back() != L'\\')
            srcBin += L'\\';
        srcBin += L"target\\debug\\";
        srcBin += EXE_NAME;
    }

    if (GetFileAttributesW(srcBin.c_str()) == INVALID_FILE_ATTRIBUTES)
    {
        if (pWindow)
            pWindow->UpdateLog(L"错误: 未找到编译输出文件");
        return false;
    }

    CreateDirectoryW(installPath, NULL);

    std::wstring destBin = installPath;
    if (!destBin.empty() && destBin.back() != L'\\')
        destBin += L'\\';
    destBin += EXE_NAME;

    if (!CopyFileW(srcBin.c_str(), destBin.c_str(), FALSE))
    {
        if (pWindow)
            pWindow->UpdateLog(L"错误: 复制文件失败");
        return false;
    }

    if (pWindow)
        pWindow->UpdateLog(L"文件复制完成");

    return true;
}

bool CreateStartMenuShortcut(const wchar_t* exePath, const wchar_t* installPath)
{
    if (!exePath || !installPath)
        return false;

    wchar_t startMenuPath[MAX_PATH];
    if (!SHGetSpecialFolderPathW(NULL, startMenuPath, CSIDL_PROGRAMS, TRUE))
        return false;

    std::wstring shortcutPath = startMenuPath;
    if (!shortcutPath.empty() && shortcutPath.back() != L'\\')
        shortcutPath += L'\\';
    shortcutPath += L"Linux 系统监控";
    shortcutPath += L".lnk";

    IShellLinkW* pShellLink = NULL;
    HRESULT hr = CoCreateInstance(CLSID_ShellLink, NULL, CLSCTX_INPROC_SERVER,
        IID_IShellLinkW, (void**)&pShellLink);
    if (FAILED(hr) || !pShellLink)
        return false;

    pShellLink->SetPath(exePath);
    pShellLink->SetDescription(L"Linux 系统监控");

    IPersistFile* pPersistFile = NULL;
    hr = pShellLink->QueryInterface(IID_IPersistFile, (void**)&pPersistFile);
    if (SUCCEEDED(hr) && pPersistFile)
    {
        pPersistFile->Save(shortcutPath.c_str(), TRUE);
        pPersistFile->Release();
    }

    pShellLink->Release();
    return true;
}

bool CreateUninstallBatch(const wchar_t* installPath)
{
    if (!installPath)
        return false;

    std::wstring uninstallBat = installPath;
    if (!uninstallBat.empty() && uninstallBat.back() != L'\\')
        uninstallBat += L'\\';
    uninstallBat += L"uninstall.bat";

    HANDLE hFile = CreateFileW(uninstallBat.c_str(), GENERIC_WRITE, 0, NULL,
        CREATE_ALWAYS, FILE_ATTRIBUTE_HIDDEN, NULL);
    if (hFile == INVALID_HANDLE_VALUE)
        return false;

    const wchar_t content[] =
        L"@echo off\r\n"
        L"echo 正在卸载 Linux 系统监控...\r\n"
        L"timeout /t 2 /nobreak > nul\r\n"
        L"del /f /q \"%~dp0linux-sys-monitor.exe\" 2>nul\r\n"
        L"del \"%~dp0uninstall.bat\" 2>nul\r\n"
        L"echo 卸载完成\r\n"
        L"pause\r\n";

    DWORD written;
    WriteFile(hFile, content, (DWORD)(wcslen(content) * sizeof(wchar_t)), &written, NULL);
    CloseHandle(hFile);

    return true;
}

bool AddToAutoStart(const wchar_t* exePath)
{
    if (!exePath)
        return false;

    HKEY hKey;
    LONG result = RegCreateKeyExW(HKEY_CURRENT_USER,
        L"Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        0, NULL, 0, KEY_WRITE, NULL, &hKey, NULL);

    if (result != ERROR_SUCCESS)
        return false;

    RegSetValueExW(hKey, L"LinuxSysMonitor", 0, REG_SZ,
        (const BYTE*)exePath, (DWORD)((wcslen(exePath) + 1) * sizeof(wchar_t)));

    RegCloseKey(hKey);
    return true;
}

void RunInstallation(InstallerWindow* pWindow)
{
    if (!pWindow)
        return;

    std::wstring installPath = pWindow->GetInstallPath();
    if (installPath.empty())
        installPath = L"C:\\Program Files\\LinuxSysMonitor";

    std::wstring tempPath = GetTempPath();
    std::wstring repoPath = tempPath + L"linux-sys-monitor";

    CreateDirectoryW(tempPath.c_str(), NULL);
    RemoveDirectoryW(repoPath.c_str());

    if (!CloneRepository(repoPath.c_str(), pWindow))
    {
        pWindow->PostInstallError(L"代码仓库克隆失败");
        return;
    }

    if (!BuildProject(repoPath.c_str(), pWindow))
    {
        pWindow->PostInstallError(L"项目编译失败");
        return;
    }

    if (!CopyBinary(repoPath.c_str(), installPath.c_str(), pWindow))
    {
        pWindow->PostInstallError(L"文件复制失败");
        return;
    }

    std::wstring exePath = installPath;
    if (!exePath.empty() && exePath.back() != L'\\')
        exePath += L'\\';
    exePath += EXE_NAME;

    CreateStartMenuShortcut(exePath.c_str(), installPath.c_str());
    CreateUninstallBatch(installPath.c_str());

    pWindow->SetAutoLaunch(false);
    pWindow->PostInstallComplete();

    SHELLEXECUTEINFOW shExecInfo;
    memset(&shExecInfo, 0, sizeof(shExecInfo));
    shExecInfo.cbSize = sizeof(SHELLEXECUTEINFOW);
    shExecInfo.fMask = SEE_MASK_NOCLOSEPROCESS;
    shExecInfo.hwnd = NULL;
    shExecInfo.lpVerb = L"explorer";
    shExecInfo.lpFile = tempPath.c_str();
    shExecInfo.nShow = SW_HIDE;
    shExecInfo.hInstApp = NULL;
    ShellExecuteExW(&shExecInfo);

    if (shExecInfo.hProcess)
    {
        WaitForSingleObject(shExecInfo.hProcess, 5000);
        CloseHandle(shExecInfo.hProcess);
    }

    RemoveDirectoryW(repoPath.c_str());
}
