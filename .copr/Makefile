srpm:
	dnf -y install cargo
	cargo install cargo-rpm
	cargo rpm build
	mv target/release/rpmbuild/SRPMS/* $(outdir)
