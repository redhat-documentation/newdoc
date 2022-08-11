srpm:
	dnf install cargo -y
	cargo install cargo-rpm --profile=dev
	cargo rpm build
	mv target/release/rpmbuild/SRPMS/* $(outdir)
