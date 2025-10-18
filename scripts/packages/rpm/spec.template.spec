Name:           __NAME__
Version:        __VERSION__
Release:        1%{?dist}
Summary:        VEM (Vim Environment Manager)

License:        MIT
URL:            https://github.com/ryo-arima/vem
BuildArch:      %{_target_cpu}

%description
VEM is a tool to manage multiple Vim environments.

%install
# Files are pre-staged into %{buildroot} by the external packaging script.

%files
/usr/local/bin/__NAME__