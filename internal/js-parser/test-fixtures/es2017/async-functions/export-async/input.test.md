# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2017 > async-functions > export-async`

### `ast`

```javascript
JSRoot {
	body: [
		JSExportLocalDeclaration {
			exportKind: "value"
			specifiers: []
			loc: SourceLocation es2017/async-functions/export-async/input.js 1:0-1:6
		}
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "async"
				loc: SourceLocation es2017/async-functions/export-async/input.js 1:7-1:12 (async)
			}
			loc: SourceLocation es2017/async-functions/export-async/input.js 1:7-1:13
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: []
				category: ["parse"]
				categoryValue: "js"
				message: RAW_MARKUP {value: "Started with `export async` so we expected to receive an async function but no function keyword was found"}
			}
			location: {
				language: "js"
				path: UIDPath<es2017/async-functions/export-async/input.js>
				end: Position 1:13
				start: Position 1:12
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: []
	path: UIDPath<es2017/async-functions/export-async/input.js>
	loc: SourceLocation es2017/async-functions/export-async/input.js 1:0-2:0
}
```

### `diagnostics`

```

 es2017/async-functions/export-async/input.js:1:12 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Started with `export async` so we expected to receive an async function but no function
    keyword was found

    export async;
                ^


```