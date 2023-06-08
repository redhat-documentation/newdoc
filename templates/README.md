# templates

This directory contains the templates that `newdoc` uses at runtime to generate new files.

The templates are AsciiDoc files with additional directives for `newdoc` to replace certain parts of the content with generated text. Currently, `newdoc` uses the [Askama](https://github.com/djc/askama/) templating library to perform the substitutions in the templates.

For more information on the Askama templating syntax, see [Template Syntax](https://djc.github.io/askama/template_syntax.html).

## What about the context attribute in the assembly?

The assembly template features a complicated configuration of the context attribute. In the order of appearance, as found at the time of commit `8b7fed5e370e6448928d95d68447e444e82c397c`:


- ```asciidoc
  ifdef::context[:parent-context-of-{{module_anchor}}: {context}]
  ```

    If context is already defined before this assembly starts, save the original context value to a new attribute. The assembly will restore the original context at the end of the file.

- ```asciidoc
  ifndef::context[]
  [id="{{module_anchor}}"]
  endif::[]
  ifdef::context[]
  [id="{{module_anchor}}_{context}"]
  endif::[]
  = {{module_title}}
  ```

    If context is already defined before this assembly starts, append the original context to the end of the assembly ID. This is similar to the behavior of modules: a module ID is always composed of the name of the module itself and the ID of the assembly that includes the module.

    However, an assembly can stand on its own, or it can be included from the root of the guide where context might not be defined. in that situation, the assembly ID without context would end up in the form of `{{module_anchor}}_`, ending in an underscore. This is an invalid ID in some build systems.

    To work around the build system limitation, check whether context exists. If it doesn't exist, use an ID that is only based on the assembly name, without the context part.

    - Q: Why does an assembly even need context in its ID?

    - A: For the same reason as a module: So that you can reuse the assembly several times at different places of a guide.

- ```asciidoc
  :context: {{module_anchor}}
  ```

    Set a context attribute that modules inside of this assembly pick up and use in their IDs.

- ```asciidoc
  ifdef::parent-context-of-{{module_anchor}}[:context: {parent-context-of-{{module_anchor}}}]
  ifndef::parent-context-of-{{module_anchor}}[:!context:]
  ```
    When the assembly ends, restore the original context back to its value that it had before this assembly started.

    This step encapsulates the context just to the assembly, and as a result, all assemblies included on the same level inherit the same context attribute. For example, if a title defined context `A` and includes several assemblies, all of them use `A` in their IDs. Without this restoration step, each assembly would instead use the context of the previous assembly in the guide.
