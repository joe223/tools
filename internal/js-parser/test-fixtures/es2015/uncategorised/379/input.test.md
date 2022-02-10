# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > uncategorised > 379`

### `ast`

```javascript
JSRoot {
	body: [
		JSClassDeclaration {
			id: JSBindingIdentifier {
				name: "enum"
				loc: SourceLocation es2015/uncategorised/379/input.js 1:6-1:10 (enum)
			}
			meta: JSClassHead {
				body: []
				loc: SourceLocation es2015/uncategorised/379/input.js 1:0-1:13
			}
			loc: SourceLocation es2015/uncategorised/379/input.js 1:0-1:13
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {advice: [], category: ["parse"], categoryValue: "js", message: ["enum", RAW_MARKUP {value: " is a reserved word"}]}
			location: {
				language: "js"
				path: UIDPath<es2015/uncategorised/379/input.js>
				end: Position 1:10
				start: Position 1:6
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: []
	path: UIDPath<es2015/uncategorised/379/input.js>
	loc: SourceLocation es2015/uncategorised/379/input.js 1:0-2:0
}
```

### `diagnostics`

```

 es2015/uncategorised/379/input.js:1:6 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ enum is a reserved word

    class enum {}
          ^^^^


```