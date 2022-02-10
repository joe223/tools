# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > identifiers > invalid-escape-seq-true`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSBooleanLiteral {
				value: true
				loc: SourceLocation es2015/identifiers/invalid-escape-seq-true/input.js 1:0-1:9
			}
			loc: SourceLocation es2015/identifiers/invalid-escape-seq-true/input.js 1:0-1:9
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
				message: [RAW_MARKUP {value: "Escape sequence in keyword <emphasis>"}, "true", RAW_MARKUP {value: "</emphasis>"}]
			}
			location: {
				language: "js"
				path: UIDPath<es2015/identifiers/invalid-escape-seq-true/input.js>
				end: Position 1:0
				start: Position 1:0
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/identifiers/invalid-escape-seq-true/input.js>
	loc: SourceLocation es2015/identifiers/invalid-escape-seq-true/input.js 1:0-2:0
}
```

### `diagnostics`

```

 es2015/identifiers/invalid-escape-seq-true/input.js:1 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Escape sequence in keyword true

    \u0074rue
    ^


```