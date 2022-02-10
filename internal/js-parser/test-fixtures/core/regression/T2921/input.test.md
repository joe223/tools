# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > regression > T2921`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSBinaryExpression {
				operator: "<="
				left: JSReferenceIdentifier {
					name: "a"
					loc: SourceLocation core/regression/T2921/input.js 1:0-1:1 (a)
				}
				right: JSReferenceIdentifier {
					name: "INVALID_PLACEHOLDER"
					loc: SourceLocation core/regression/T2921/input.js 1:4-1:5
				}
				loc: SourceLocation core/regression/T2921/input.js 1:0-1:5
			}
			loc: SourceLocation core/regression/T2921/input.js 1:0-1:5
		}
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "b"
				loc: SourceLocation core/regression/T2921/input.js 1:6-1:7 (b)
			}
			loc: SourceLocation core/regression/T2921/input.js 1:6-1:8
		}
	]
	comments: []
	corrupt: true
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {advice: [], category: ["parse"], categoryValue: "js", message: [RAW_MARKUP {value: "Unknown start to an "}, "statement expression"]}
			location: {
				language: "js"
				path: UIDPath<core/regression/T2921/input.js>
				end: Position 1:4
				start: Position 1:4
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/regression/T2921/input.js>
	loc: SourceLocation core/regression/T2921/input.js 1:0-2:0
}
```

### `diagnostics`

```

 core/regression/T2921/input.js:1:4 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unknown start to an statement expression

    a <== b;
        ^


```