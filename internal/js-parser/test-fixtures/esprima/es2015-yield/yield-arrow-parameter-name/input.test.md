# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-yield > yield-arrow-parameter-name`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSArrowFunctionExpression {
				body: JSNumericLiteral {
					value: 42
					loc: SourceLocation esprima/es2015-yield/yield-arrow-parameter-name/input.js 1:11-1:13
				}
				head: JSFunctionHead {
					async: false
					hasHoistedVars: false
					params: [
						JSBindingIdentifier {
							name: "yield"
							loc: SourceLocation esprima/es2015-yield/yield-arrow-parameter-name/input.js 1:1-1:6 (yield)
						}
					]
					loc: SourceLocation esprima/es2015-yield/yield-arrow-parameter-name/input.js 1:0-1:10
				}
				loc: SourceLocation esprima/es2015-yield/yield-arrow-parameter-name/input.js 1:0-1:13
			}
			loc: SourceLocation esprima/es2015-yield/yield-arrow-parameter-name/input.js 1:0-1:14
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/es2015-yield/yield-arrow-parameter-name/input.js>
	loc: SourceLocation esprima/es2015-yield/yield-arrow-parameter-name/input.js 1:0-2:0
}
```

### `diagnostics`

```

```