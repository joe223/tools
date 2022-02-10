# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `experimental > module-attributes > valid-syntax-with-attributes-and-value`

### `ast`

```javascript
JSRoot {
	body: [
		JSImportDeclaration {
			namedSpecifiers: []
			source: JSStringLiteral {
				value: "x"
				loc: SourceLocation experimental/module-attributes/valid-syntax-with-attributes-and-value/input.js 1:7-1:10
			}
			loc: SourceLocation experimental/module-attributes/valid-syntax-with-attributes-and-value/input.js 1:0-1:10
		}
		JSWithStatement {
			body: JSExpressionStatement {
				expression: JSReferenceIdentifier {
					name: "INVALID_PLACEHOLDER"
					loc: SourceLocation experimental/module-attributes/valid-syntax-with-attributes-and-value/input.js 1:20-1:21
				}
				loc: SourceLocation experimental/module-attributes/valid-syntax-with-attributes-and-value/input.js 1:20-1:21
			}
			object: JSReferenceIdentifier {
				name: "type"
				loc: SourceLocation experimental/module-attributes/valid-syntax-with-attributes-and-value/input.js 1:16-1:20 (type)
			}
			loc: SourceLocation experimental/module-attributes/valid-syntax-with-attributes-and-value/input.js 1:11-1:21
		}
		JSExpressionStatement {
			expression: JSMemberExpression {
				object: JSStringLiteral {
					value: "json"
					loc: SourceLocation experimental/module-attributes/valid-syntax-with-attributes-and-value/input.js 1:22-1:28
				}
				property: JSComputedMemberProperty {
					value: JSNumericLiteral {
						value: 0
						loc: SourceLocation experimental/module-attributes/valid-syntax-with-attributes-and-value/input.js 2:1-2:2
					}
					loc: SourceLocation experimental/module-attributes/valid-syntax-with-attributes-and-value/input.js 2:0-2:3
				}
				loc: SourceLocation experimental/module-attributes/valid-syntax-with-attributes-and-value/input.js 1:22-2:3
			}
			loc: SourceLocation experimental/module-attributes/valid-syntax-with-attributes-and-value/input.js 1:22-2:3
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
				path: UIDPath<experimental/module-attributes/valid-syntax-with-attributes-and-value/input.js>
				end: Position 1:10
				start: Position 1:11
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: []
	path: UIDPath<experimental/module-attributes/valid-syntax-with-attributes-and-value/input.js>
	loc: SourceLocation experimental/module-attributes/valid-syntax-with-attributes-and-value/input.js 1:0-3:0
}
```

### `diagnostics`

```

 experimental/module-attributes/valid-syntax-with-attributes-and-value/input.js:1:11 parse(js) ━━━━━

  ✖ Expected a semicolon or a line terminator

  > 1 │ import "x" with type: "json"
      │            ^
    2 │ [0]


```