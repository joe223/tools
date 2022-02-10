# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > invalid-syntax > migrated_0010`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSNumericLiteral {
				value: 1
				format: "octal"
				loc: SourceLocation esprima/invalid-syntax/migrated_0010/input.js 1:0-1:2
			}
			loc: SourceLocation esprima/invalid-syntax/migrated_0010/input.js 1:0-1:2
		}
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "a"
				loc: SourceLocation esprima/invalid-syntax/migrated_0010/input.js 1:2-1:3 (a)
			}
			loc: SourceLocation esprima/invalid-syntax/migrated_0010/input.js 1:2-1:3
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
				message: RAW_MARKUP {value: "Identifier directly after number"}
			}
			location: {
				language: "js"
				path: UIDPath<esprima/invalid-syntax/migrated_0010/input.js>
				end: Position 1:2
				start: Position 1:2
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/invalid-syntax/migrated_0010/input.js>
	loc: SourceLocation esprima/invalid-syntax/migrated_0010/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/invalid-syntax/migrated_0010/input.js:1:2 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Identifier directly after number

    01a
      ^


```