# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > cast > as`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: TSAsExpression {
				expression: JSReferenceIdentifier {
					name: "x"
					loc: SourceLocation typescript/cast/as/input.ts 1:0-1:1 (x)
				}
				typeAnnotation: TSTypeReference {
					typeName: JSReferenceIdentifier {
						name: "T"
						loc: SourceLocation typescript/cast/as/input.ts 1:5-1:6 (T)
					}
					loc: SourceLocation typescript/cast/as/input.ts 1:5-1:6
				}
				loc: SourceLocation typescript/cast/as/input.ts 1:0-1:6
			}
			loc: SourceLocation typescript/cast/as/input.ts 1:0-1:7
		}
		JSExpressionStatement {
			trailingComments: ["0"]
			expression: TSAsExpression {
				expression: JSBinaryExpression {
					operator: "<"
					left: JSReferenceIdentifier {
						name: "x"
						loc: SourceLocation typescript/cast/as/input.ts 2:0-2:1 (x)
					}
					right: JSReferenceIdentifier {
						name: "y"
						loc: SourceLocation typescript/cast/as/input.ts 2:4-2:5 (y)
					}
					loc: SourceLocation typescript/cast/as/input.ts 2:0-2:5
				}
				typeAnnotation: TSBooleanKeywordTypeAnnotation {
					loc: SourceLocation typescript/cast/as/input.ts 2:9-2:16
				}
				loc: SourceLocation typescript/cast/as/input.ts 2:0-2:16
			}
			loc: SourceLocation typescript/cast/as/input.ts 2:0-2:17
		}
		JSExpressionStatement {
			leadingComments: ["0"]
			trailingComments: ["1"]
			expression: JSBinaryExpression {
				operator: "==="
				left: JSReferenceIdentifier {
					name: "x"
					loc: SourceLocation typescript/cast/as/input.ts 3:0-3:1 (x)
				}
				right: TSAsExpression {
					expression: JSNumericLiteral {
						value: 1
						loc: SourceLocation typescript/cast/as/input.ts 3:6-3:7
					}
					typeAnnotation: TSNumberKeywordTypeAnnotation {
						loc: SourceLocation typescript/cast/as/input.ts 3:11-3:17
					}
					loc: SourceLocation typescript/cast/as/input.ts 3:6-3:17
				}
				loc: SourceLocation typescript/cast/as/input.ts 3:0-3:17
			}
			loc: SourceLocation typescript/cast/as/input.ts 3:0-3:18
		}
		JSExpressionStatement {
			leadingComments: ["1"]
			expression: TSAsExpression {
				expression: TSAsExpression {
					expression: JSReferenceIdentifier {
						name: "x"
						loc: SourceLocation typescript/cast/as/input.ts 4:0-4:1 (x)
					}
					typeAnnotation: TSAnyKeywordTypeAnnotation {
						loc: SourceLocation typescript/cast/as/input.ts 4:5-4:8
					}
					loc: SourceLocation typescript/cast/as/input.ts 4:0-4:8
				}
				typeAnnotation: TSTypeReference {
					typeName: JSReferenceIdentifier {
						name: "T"
						loc: SourceLocation typescript/cast/as/input.ts 4:12-4:13 (T)
					}
					loc: SourceLocation typescript/cast/as/input.ts 4:12-4:13
				}
				loc: SourceLocation typescript/cast/as/input.ts 4:0-4:13
			}
			loc: SourceLocation typescript/cast/as/input.ts 4:0-4:14
		}
	]
	comments: [
		CommentLine {
			id: "0"
			value: " (x < y) as boolean;"
			loc: SourceLocation typescript/cast/as/input.ts 2:18-2:40
		}
		CommentLine {
			id: "1"
			value: " x === (1 as number);"
			loc: SourceLocation typescript/cast/as/input.ts 3:19-3:42
		}
	]
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/cast/as/input.ts>
	loc: SourceLocation typescript/cast/as/input.ts 1:0-5:0
}
```

### `diagnostics`

```

```