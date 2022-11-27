# swc plugin remove-console

A swc plugin helps you remove your `console.*` code

Notice `*` now support:

- `log`
- `warn`
- `error`
- `info`

## Configure

First ensure you've installed `swc` and `swc-plugin-remove-console`, then add following these codes in your `.swcrc` file

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "swc-plugin-remove-console",
          {
            "exclude": ["error"] // this config is optional, default is [], you can pass `log`, `warn`, `error`, `info` to exclude some of them
          }
        ]
      ]
    }
  }
}

```
