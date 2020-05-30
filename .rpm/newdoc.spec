%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: newdoc
Summary: Generate an AsciiDoc file using a modular template
Version: @@VERSION@@
Release: @@RELEASE@@
License: GPLv3+
Group: Applications/Text
Source0: %{name}-%{version}.tar.gz

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
The newdoc tool generates pre-populated module and assembly files formatted with AsciiDoc, which are used in Red Hat and Fedora documentation. The generated files follow the template guidelines maintained by the Modular Documentation initiative: https://redhat-documentation.github.io/modular-docs/.

%prep
%setup -q

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
