Name:           __NAME__
Version:        __VERSION__
Release:        1%{?dist}
Summary:        VEM (Vim Environment Manager)

License:        MIT
URL:            https://github.com/ryo-arima/vem
BuildArch:      %{_target_cpu}
Source0:        __NAME__

%description
VEM is a tool to manage multiple Vim environments.

%install
mkdir -p %{buildroot}/usr/local/bin
install -m 0755 %{SOURCE0} %{buildroot}/usr/local/bin/__NAME__

%files
/usr/local/bin/__NAME__