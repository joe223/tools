# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > arrow-function > optional-parameter`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSArrowFunctionExpression {
				body: JSReferenceIdentifier {
					name: "x"
					loc: SourceLocation typescript/arrow-function/optional-parameter/input.ts 1:21-1:22 (x)
				}
				head: JSFunctionHead {
					async: false
					hasHoistedVars: false
					params: [
						JSBindingIdentifier {
							name: "x"
							meta: JSPatternMeta {
								optional: true
								typeAnnotation: TSNumberKeywordTypeAnnotation {
									loc: SourceLocation typescript/arrow-function/optional-parameter/input.ts 1:5-1:11
								}
								loc: SourceLocation typescript/arrow-function/optional-parameter/input.ts 1:21-1:20
							}
							loc: SourceLocation typescript/arrow-function/optional-parameter/input.ts 1:21-1:20
						}
					]
					returnType: TSAnyKeywordTypeAnnotation {
						loc: SourceLocation typescript/arrow-function/optional-parameter/input.ts 1:14-1:17
					}
					loc: SourceLocation typescript/arrow-function/optional-parameter/input.ts 1:0-1:20
				}
				loc: SourceLocation typescript/arrow-function/optional-parameter/input.ts 1:0-1:22
			}
			loc: SourceLocation typescript/arrow-function/optional-parameter/input.ts 1:0-1:23
		}
		JSExpressionStatement {
			expression: JSCallExpression {
				arguments: []
				callee: JSArrowFunctionExpression {
					body: JSBinaryExpression {
						operator: "+"
						left: JSReferenceIdentifier {
							name: "k"
							loc: SourceLocation typescript/arrow-function/optional-parameter/input.ts 2:9-2:10 (k)
						}
						right: JSNumericLiteral {
							value: 1
							loc: SourceLocation typescript/arrow-function/optional-parameter/input.ts 2:13-2:14
						}
						loc: SourceLocation typescript/arrow-function/optional-parameter/input.ts 2:9-2:14
					}
					head: JSFunctionHead {
						async: false
						hasHoistedVars: false
						params: [
							JSBindingIdentifier {
								name: "k"
								meta: JSPatternMeta {
									optional: true
									loc: SourceLocation typescript/arrow-function/optional-parameter/input.ts 2:9-2:8
								}
								loc: SourceLocation typescript/arrow-function/optional-parameter/input.ts 2:9-2:8
							}
						]
						loc: SourceLocation typescript/arrow-function/optional-parameter/input.ts 2:1-2:8
					}
					loc: SourceLocation typescript/arrow-function/optional-parameter/input.ts 2:1-2:14
				}
				loc: SourceLocation typescript/arrow-function/optional-parameter/input.ts 2:0-2:17
			}
			loc: SourceLocation typescript/arrow-function/optional-parameter/input.ts 2:0-2:18
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/arrow-function/optional-parameter/input.ts>
	loc: SourceLocation typescript/arrow-function/optional-parameter/input.ts 1:0-3:0
}
```

### `diagnostics`

```

```