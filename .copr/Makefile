srpm:
	dnf install -y rpmdevtools cargo
	rpmdev-setuptree
	cargo package
	mv target/package/*.crate ~/rpmbuild/SOURCES/
	cp newdoc.spec ~/rpmbuild/SPECS/
	rpmbuild -bs ~/rpmbuild/SPECS/newdoc.spec
	cp ~/rpmbuild/SRPMS/newdoc-*.src.rpm $(outdir)
