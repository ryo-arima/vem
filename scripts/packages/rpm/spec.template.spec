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
mkdir -p %{buildroot}/usr/local/bin
install -m 0755 __NAME__ %{buildroot}/usr/local/bin/__NAME__

%files
/usr/local/bin/__NAME__

%changelog
* Thu Jan 01 1970 Package Bot - __VERSION__-1
- Initial package