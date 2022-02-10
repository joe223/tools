# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > invalid-syntax > migrated_0114`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "a"
				loc: SourceLocation esprima/invalid-syntax/migrated_0114/input.js 1:0-1:1 (a)
			}
			loc: SourceLocation esprima/invalid-syntax/migrated_0114/input.js 1:0-1:1
		}
		JSIfStatement {
			consequent: JSExpressionStatement {
				expression: JSReferenceIdentifier {
					name: "INVALID_PLACEHOLDER"
					loc: SourceLocation esprima/invalid-syntax/migrated_0114/input.js 2:0-2:0
				}
				loc: SourceLocation esprima/invalid-syntax/migrated_0114/input.js 2:0-2:0
			}
			test: JSReferenceIdentifier {
				name: "INVALID_PLACEHOLDER"
				loc: SourceLocation esprima/invalid-syntax/migrated_0114/input.js 1:4-1:5
			}
			loc: SourceLocation esprima/invalid-syntax/migrated_0114/input.js 1:2-2:0
		}
	]
	comments: []
	corrupt: true
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: []
				category: ["parse"]
				categoryValue: "js"
				message: RAW_MARKUP {value: "Expected a semicolon or a line terminator"}
			}
			location: {
				language: "js"
				path: UIDPath<esprima/invalid-syntax/migrated_0114/input.js>
				end: Position 1:1
				start: Position 1:2
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/invalid-syntax/migrated_0114/input.js>
	loc: SourceLocation esprima/invalid-syntax/migrated_0114/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/invalid-syntax/migrated_0114/input.js:1:2 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Expected a semicolon or a line terminator

    a if;
      ^


```