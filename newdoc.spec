Name: newdoc
Summary: Generate an AsciiDoc file using a modular template
Version: 2.11.0
Release: 1%{?dist}
License: GPLv3+
URL: https://github.com/redhat-documentation/newdoc
Group: Applications/Text
Obsoletes: python3-newdoc, python2-newdoc
Source0: https://static.crates.io/crates/%{name}/%{name}-%{version}.crate
#Source1: https://github.com/redhat-documentation/%{name}/archive/refs/tags/v%{version}.tar.gz

ExclusiveArch: %{rust_arches}

BuildRequires: rust
BuildRequires: cargo

%description
The newdoc tool generates pre-populated module and assembly files formatted with AsciiDoc, which are used in Red Hat and Fedora documentation. The generated files follow the template guidelines maintained by the Modular Documentation initiative: https://redhat-documentation.github.io/modular-docs/.

%global debug_package %{nil}

%prep
%setup -q

%build
cargo build --release

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}%{_bindir}
#cargo install --path . --root %{buildroot}
install -m 0755 target/release/%{name} %{buildroot}%{_bindir}/%{name}

%clean
rm -rf %{buildroot}

%files
%doc README.md
%doc CHANGELOG.md
%license LICENSE
%{_bindir}/%{name}
