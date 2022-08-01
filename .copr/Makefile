srpm:
	dnf install 'dnf-command(copr)' -y
	dnf copr enable mareksu/cargo-rpm -y
	dnf install cargo cargo-rpm -y
	cargo rpm build
	mv target/release/rpmbuild/SRPMS/* $(outdir)
