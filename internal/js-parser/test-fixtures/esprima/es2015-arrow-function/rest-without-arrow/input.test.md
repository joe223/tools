# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-arrow-function > rest-without-arrow`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSBinaryExpression {
				operator: "+"
				left: JSReferenceIdentifier {
					name: "INVALID_PLACEHOLDER"
					loc: SourceLocation esprima/es2015-arrow-function/rest-without-arrow/input.js 1:7-1:6
				}
				right: JSNumericLiteral {
					value: 1
					loc: SourceLocation esprima/es2015-arrow-function/rest-without-arrow/input.js 1:9-1:10
				}
				loc: SourceLocation esprima/es2015-arrow-function/rest-without-arrow/input.js 1:0-1:10
			}
			loc: SourceLocation esprima/es2015-arrow-function/rest-without-arrow/input.js 1:0-1:10
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
				message: [RAW_MARKUP {value: "Unexpected character <emphasis>"}, ".", RAW_MARKUP {value: "</emphasis>"}]
			}
			location: {
				language: "js"
				path: UIDPath<esprima/es2015-arrow-function/rest-without-arrow/input.js>
				end: Position 1:1
				start: Position 1:1
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/es2015-arrow-function/rest-without-arrow/input.js>
	loc: SourceLocation esprima/es2015-arrow-function/rest-without-arrow/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/es2015-arrow-function/rest-without-arrow/input.js:1:1 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unexpected character .

    (...a) + 1
     ^


```