# Packaging newdoc

The following are instructions for the maintainers of `newdoc` to package and distribute new releases.

## Packaging and distributing newdoc as an RPM package

1. Install the  `cargo` package manager. For details, see <https://rustup.rs/>.

2. Install the `cargo-rpm` extension:

    ```
    $ cargo install cargo-rpm
    ```

3. In the `newdoc` project directory, build RPM packages:

    ```
    $ cargo rpm build
    ```

    This command build packages in the `target/release/rpmbuild/` directory.

4. Log into the Copr repository administration: <https://copr.fedorainfracloud.org/coprs/mareksu/newdoc-rs/>.

5. Go to the **Builds** tab.

6. Click **New Build** and select **Upload**.

7. In the **Provide the source** section, upload the most recent SRPM package from the `target/release/rpmbuild/SRPMS/` directory.

8. Click **Build**.

## Packaging and distributing newdoc on Crates.io

1. If you are publishing to Crates.io for teh first time on this system, log into your account:

    ```
    $ cargo login
    ```

    You can manage your login tokens in your account settings: <https://crates.io/me>.

2. Publish the latest version of `newdoc` to Crates.io:

    ```
    $ cargo publish
    ```


## Packaging and distributing newdoc as a Docker image

Note: The following steps might be sub-optimal. Feel free to suggest improvements.

1. Install the `docker` or `podman` tool.

    If you use `podman`, replace `docker` with `podman` in the following commands.

2. Log into the Docker Hub account:

    ```
    $ docker login --username mrksu docker.io
    ```

3. Build a new image. For example:

    ```
    $ docker build -t mrksu/newdoc:v2.3.3 .
    ```

4. Find the Image ID of the built image:

    ```
    $ docker images
    ```

5. Tag the new version. For example:

    ```
    $ docker tag 390e73cb470d mrksu/newdoc:v2.3.3
    ```

6. Upload the new image:

    ```
    $ docker push mrksu/newdoc:v2.3.3
    ```