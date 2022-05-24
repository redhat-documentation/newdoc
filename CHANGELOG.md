# Changelog

The following is a summary of changes in each `newdoc` release, which is also a Git tag by the same name in this repository.

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

* Update the version to v2.3.4
* Updated README
* New packaging options
* Enabled CI

## v2.3.3

* Make context in assembly IDs optional and conditional

## v2.3.2

* Align the Optional formatting with the IBM Style Guide; #8

## v2.3.1

* Use colors instead of emoji for highlighting in the output

## v2.3.0

* Update the version

## v2.2.2

* Delete a print statement left in by mistake. Update the version

## v2.2.1

* Update the version

## v2.2.0

* New release

## v2.1.1

* Update the version

## v2.1.0

* Update the version

## v2.0.0

* Version updated in the lock file

