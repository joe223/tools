# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > uncategorised > 226`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "default"
				loc: SourceLocation es2015/uncategorised/226/input.js 1:9-1:16 (default)
			}
			body: JSBlockStatement {
				body: []
				directives: []
				loc: SourceLocation es2015/uncategorised/226/input.js 1:19-1:21
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: false
				params: []
				loc: SourceLocation es2015/uncategorised/226/input.js 1:16-1:18
			}
			loc: SourceLocation es2015/uncategorised/226/input.js 1:0-1:21
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
				message: [RAW_MARKUP {value: "Unexpected keyword <emphasis>"}, "default", RAW_MARKUP {value: "</emphasis>"}]
			}
			location: {
				language: "js"
				path: UIDPath<es2015/uncategorised/226/input.js>
				end: Position 1:16
				start: Position 1:9
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/uncategorised/226/input.js>
	loc: SourceLocation es2015/uncategorised/226/input.js 1:0-1:21
}
```

### `diagnostics`

```

 es2015/uncategorised/226/input.js:1:9 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unexpected keyword default

    function default() {}
             ^^^^^^^


```