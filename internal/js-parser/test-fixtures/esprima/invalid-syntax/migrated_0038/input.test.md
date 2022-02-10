# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > invalid-syntax > migrated_0038`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSRegExpLiteral {
				global: false
				insensitive: false
				multiline: false
				noDotNewline: false
				sticky: false
				unicode: false
				expression: JSRegExpSubExpression {
					body: []
					loc: SourceLocation esprima/invalid-syntax/migrated_0038/input.js 1:1-1:1
				}
				loc: SourceLocation esprima/invalid-syntax/migrated_0038/input.js 1:0-1:2
			}
			loc: SourceLocation esprima/invalid-syntax/migrated_0038/input.js 1:0-1:2
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
				message: RAW_MARKUP {value: "Unterminated regular expression"}
			}
			location: {
				language: "js"
				path: UIDPath<esprima/invalid-syntax/migrated_0038/input.js>
				end: Position 1:1
				start: Position 1:1
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/invalid-syntax/migrated_0038/input.js>
	loc: SourceLocation esprima/invalid-syntax/migrated_0038/input.js 1:0-1:2
}
```

### `diagnostics`

```

 esprima/invalid-syntax/migrated_0038/input.js:1:1 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unterminated regular expression

    /
     ^


```