# Changelog

The following is a summary of changes in each `newdoc` release, which is also a Git tag by the same name in this repository.

## v2.15.1

* Properly follow the `YYYY-MM-DD` date format. Fixes [issue #39](https://github.com/redhat-documentation/newdoc/issues/39).
* Update various dependencies.

## v2.15.0

* Add metadata in the generated files that specify which newdoc version generated the document and the date that it was generated.
* Build the RPM using rustup to work around the outdated toolchain on RHEL.

## v2.14.1

* Rename `_content-type` to `_mod-docs-content-type`. See [issue #37](https://github.com/redhat-documentation/newdoc/issues/37).

## v2.14.0

* Update modular templates to synchronize with [modular-docs/pull/208](https://github.com/redhat-documentation/modular-docs/pull/208/).
* Remove the legacy, deprecated `--validate` (`-l`) option. Please use Enki instead: <https://github.com/Levi-Leah/enki/>.
* The command-line options parser previously disabled snippets by mistake. Fix and re-enable them.
* Switch the main container to the Alpine base from UBI Micro.
* Update various dependencies.

## v2.13.1

* The help message and the man page now specify that the validation feature is deprecated. See [#36](https://github.com/redhat-documentation/newdoc/issues/36).

## v2.13.0

* By default, the generated files do not contain any comments. The `--no-comments` (`-C`) option is now deprecated and has no effect. You can use the new `--comments` (`-M`) option to generate files with the comments that were previously the default.

## v2.12.0

* Deprecate the `--validate` (`-l`) option. Please use `enki` instead: <https://github.com/Levi-Leah/enki/>.
* Switch to the `bpaf` command-line argument parser.

## v2.11.0

* Separate options for the module type prefix in IDs (anchors) and file names.

## v2.10.6

* A prettier confirmation prompt when overwriting a file.

## v2.10.5

* Update the modular templates to the latest upstream version.

## v2.10.4

* Improvements to error handling and reporting.

## v2.10.3

* Validation: Jupiter now supports attributes in titles.
* Remove the `--detect-directory` option, which has been the default behavior.
* Minor internal changes.

## v2.10.2

* Sanitize non-ASCII characters in the module ID ([#33](https://github.com/redhat-documentation/newdoc/issues/33)).
* No longer check for the `experimental` attribute, which isn't required anymore.

## v2.10.1

* Fix an ID bug reported by Marc Muehlfeld.

## v2.10.0

* Enable generating the snippet file type.

## v2.9.8

* Remove the abstract tag from the templates. Jupiter doesn't require it.
* No longer report a missing abstract tag in validation

## v2.9.7

* The `--validate` option can take multiple values.

## v2.9.6

* No longer validate that xrefs are path-based; Jupiter does not require them
* Changes to error handling

## v2.9.5

* Check that each module name is a single string
* Various internal improvements and documentation fixes

## v2.9.4

* Rename the `:_module-type:` attribute to `:_content-type:`

## v2.9.3

* Validation: Report when no issues have been found.
* Improve the documentation in the readme.

## v2.9.2

* Validation: Use a slightly more robust detection of path-based xrefs.

## v2.9.1

* Validation: Check that supported xrefs are path-based.

## v2.9.0

* Add a validation (linting) functionality using the `--validate` option.

## v2.8.3

* Update the modular templates to match upstream changes

## v2.8.2

* Add the module type attributes; Issue #18
* Remove the blank line after 'Additional resources' in the assembly, which caused issues with Pantheon 2

## v2.8.1

* Update the modular templates to match upstream changes

## v2.8.0

* Use a standardized syntax for configuring the templates (askama).
* Remove extra blank lines from the generated files.

## v2.7.0

* Attempt to fill in the full include path by default. This obsoletes the `--detect-directory` option. Issue #16
* Use a more standardizedterminal logging solution. You can now set the verbosity level.

## v2.6.4

* Update the modular templates.

## v2.6.3

* Update the crates that newdoc depends on.

## v2.6.2

* Bug fix: With the --no-comments option, remove all multi-line comments, not just the first one

## v2.6.1

* Change the assembly prerequisites to a numbered heading in accordance with modular-docs #134
* Small internal changes

## v2.6.0

* The templates have been updated with Patheon 2 metadata
* The generated IDs now start with a module type prefix, matching the new templates

## v2.5.0

* Add the `--no-examples` option
* Recognize block AsciiDoc comments in the templates
* Add first unit tests

## v2.4.0

* Optionally detect and fill out the include path
* Refactoring the code into smaller modules

## v2.3.5

* Deduplicate app metadata

## v2.3.4

* Updated README
* New packaging options
* Enabled CI

## v2.3.3

* Make context in assembly IDs optional and conditional

## v2.3.2

* Align the Optional formatting with the IBM Style Guide; #8

## v2.3.1

* Use colors instead of emoji for highlighting in the output

## v2.3.0 and earlier

No changelog. See the commit messages between the Git tags. 

## v2.0.0

Initial release of `newdoc` rewritten in the Rust language.

