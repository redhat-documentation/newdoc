# Packaging newdoc

The following are instructions for the maintainers of `newdoc` to package and distribute new releases.


## Preparing a new version

1. Update newdoc dependencies:

    ```
    $ cargo update
    ```

2. Make your changes to the code and merge them to the `main` branch.

3. Update the version number in `Cargo.toml` and `newdoc.spec`. The versions must be identical.

4. Commit the version update:

    ```
    $ git commit -am "Update the version to X.Y.Z"
    ```

5. Tag the latest commit with the new version number:

    ```
    $ git tag -a vX.Y.Z -m "Version X.Y.Z"
    ```

    Make sure to prefix the version in the tag name with "v" for "version".

6. Push the version tag to the remote repository:

    ```
    $ git push --follow-tags
    ```

    If you're using several remote repositories, such as origin and upstream, make sure to push the tag to all of them.


## Packaging and distributing newdoc as an RPM package

1. Log into the Copr repository administration.

    Currently, newdoc is packaged in the [mareksu/newdoc-rs](https://copr.fedorainfracloud.org/coprs/mareksu/newdoc-rs/) repository.

2. Go to the **Builds** tab.

3. Click **New Build**.

4. Select **SCM**.

5. In the **Clone url** field, paste `https://github.com/redhat-documentation/newdoc`.

6. In the **Spec File** field, use `newdoc.spec`.

7. Click **Build**.


## Packaging and distributing newdoc with Homebrew

1. Make sure you have access to the existing Homebrew repository.

    Currently, newdoc is packaged in [msuchane/homebrew-repo](https://github.com/redhat-documentation/homebrew-repo), but a fork at [redhat-documentation/homebrew-repo](https://github.com/redhat-documentation/homebrew-repo) is also available.

2. Download the `.tar.gz` archive that Github created for your latest tagged version:

    <https://github.com/redhat-documentation/newdoc/tags>

3. Calculate the SHA256 checksum of this archive:

    ```
    $ sha256sum vX.Y.Z.tar.gz
    ```

4. In the `homebrew-repo` repository, edit the `Formula/newdoc.rb` file.

5. In the `url` attribute, update the version in the URL to your latest version.

6. In the `sha256` attribute, replace the existing checksum with the new checksum that you calculated.

7. Commit and push the changes.


## Packaging and distributing newdoc on Crates.io

1. If you are publishing to Crates.io for the first time on this system, log into your account:

    ```
    $ cargo login
    ```

    You can manage your login tokens in your account settings: <https://crates.io/me>.

2. Publish the latest version of `newdoc` to Crates.io:

    ```
    $ cargo publish
    ```


<!--
Note: The configuration files for a container image are still usable in the repo, but Docker Hub no longer provides free builds, so I'm disabling this part of instructions.

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
-->
