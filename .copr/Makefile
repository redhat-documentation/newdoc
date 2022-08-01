srpm:
	dnf install cargo -y
	cargo install cargo-rpm
	cargo rpm build
	mv target/release/rpmbuild/SRPMS/* $(outdir)
