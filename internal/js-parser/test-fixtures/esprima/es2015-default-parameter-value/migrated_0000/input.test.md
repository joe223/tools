# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-default-parameter-value > migrated_0000`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSAssignmentExpression {
				operator: "="
				left: JSAssignmentIdentifier {
					name: "x"
					loc: SourceLocation esprima/es2015-default-parameter-value/migrated_0000/input.js 1:0-1:1 (x)
				}
				right: JSFunctionExpression {
					body: JSBlockStatement {
						body: []
						directives: []
						loc: SourceLocation esprima/es2015-default-parameter-value/migrated_0000/input.js 1:20-1:22
					}
					head: JSFunctionHead {
						async: false
						generator: false
						hasHoistedVars: false
						params: [
							JSBindingAssignmentPattern {
								left: JSBindingIdentifier {
									name: "y"
									meta: JSPatternMeta {
										loc: SourceLocation esprima/es2015-default-parameter-value/migrated_0000/input.js 1:13-1:14
									}
									loc: SourceLocation esprima/es2015-default-parameter-value/migrated_0000/input.js 1:13-1:14 (y)
								}
								right: JSNumericLiteral {
									value: 1
									loc: SourceLocation esprima/es2015-default-parameter-value/migrated_0000/input.js 1:17-1:18
								}
								loc: SourceLocation esprima/es2015-default-parameter-value/migrated_0000/input.js 1:13-1:18
							}
						]
						loc: SourceLocation esprima/es2015-default-parameter-value/migrated_0000/input.js 1:12-1:19
					}
					loc: SourceLocation esprima/es2015-default-parameter-value/migrated_0000/input.js 1:4-1:22
				}
				loc: SourceLocation esprima/es2015-default-parameter-value/migrated_0000/input.js 1:0-1:22
			}
			loc: SourceLocation esprima/es2015-default-parameter-value/migrated_0000/input.js 1:0-1:22
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/es2015-default-parameter-value/migrated_0000/input.js>
	loc: SourceLocation esprima/es2015-default-parameter-value/migrated_0000/input.js 1:0-2:0
}
```

### `diagnostics`

```

```