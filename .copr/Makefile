srpm:
	dnf -y install cargo
	dnf -y copr enable mareksu/cargo-rpm
	dnf -y install cargo-rpm
	cargo rpm build
	mv target/release/rpmbuild/SRPMS/* $(outdir)
