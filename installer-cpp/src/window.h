#ifndef WINDOW_H
#define WINDOW_H

#include <windows.h>
#include <string>

enum InstallerPage {
    PAGE_WELCOME,
    PAGE_DEPENDENCIES,
    PAGE_INSTALL_PATH,
    PAGE_INSTALLING,
    PAGE_FINISH
};

enum DependencyStatus {
    DEP_UNKNOWN,
    DEP_NOT_FOUND,
    DEP_FOUND,
    DEP_INSTALLING
};

struct DependencyInfo {
    DependencyStatus status;
    std::wstring version;
};

class InstallerWindow {
public:
    InstallerWindow();
    ~InstallerWindow();

    bool Create();
    void Show(int nCmdShow);
    HWND GetHWND() const { return m_hwnd; }
    HINSTANCE GetInstance() const { return m_hInstance; }

    void SetPage(InstallerPage page);
    InstallerPage GetCurrentPage() const { return m_currentPage; }

    void SetInstallPath(const std::wstring& path);
    std::wstring GetInstallPath() const { return m_installPath; }

    void SetAutoLaunch(bool autoLaunch) { m_autoLaunch = autoLaunch; }
    bool GetAutoLaunch() const { return m_autoLaunch; }

    void SetProgress(int percent);
    void UpdateLog(const wchar_t* message);
    void SetStatusText(const wchar_t* text);
    void EnableControls(bool enable);
    void SetRustStatus(DependencyStatus status, const wchar_t* version = L"");
    void SetGitStatus(DependencyStatus status, const wchar_t* version = L"");

    void PostInstallComplete();
    void PostInstallError(const wchar_t* error);
    void PostDepCheckDone();

    void OnNextClicked();
    void OnBackClicked();
    void OnCancelClicked();
    void OnFinishClicked();
    void OnBrowseClicked();
    void OnInstallRustClicked();
    void OnInstallGitClicked();

    static InstallerWindow* GetFromHwnd(HWND hwnd);

private:
    void CreateControls();
    void ShowPage(InstallerPage page);
    void HideAllPages();

    void ShowWelcomePage();
    void ShowDependenciesPage();
    void ShowPathPage();
    void ShowInstallingPage();
    void ShowFinishPage();

    void StartInstallation();
    static DWORD WINAPI InstallationThread(LPVOID lpParam);

    HWND m_hwnd;
    HINSTANCE m_hInstance;
    InstallerPage m_currentPage;
    std::wstring m_installPath;
    bool m_autoLaunch;

    HWND m_hTitle;
    HWND m_hSubtitle;
    HWND m_hWelcomeText;
    HWND m_hStatusText;
    HWND m_hProgressBar;
    HWND m_hLogText;
    HWND m_hPathEdit;
    HWND m_hBrowseBtn;
    HWND m_hNextBtn;
    HWND m_hBackBtn;
    HWND m_hCancelBtn;
    HWND m_hFinishBtn;
    HWND m_hLaunchCheckbox;
    HWND m_hRustStatus;
    HWND m_hGitStatus;
    HWND m_hInstallRustBtn;
    HWND m_hInstallGitBtn;

    DependencyInfo m_rustInfo;
    DependencyInfo m_gitInfo;

    bool m_installationStarted;
};

#endif
