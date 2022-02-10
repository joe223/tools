# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `experimental > class-private-properties > await-in-private-property-in-async`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSArrowFunctionExpression {
				body: JSBlockStatement {
					body: [
						JSClassDeclaration {
							id: JSBindingIdentifier {
								name: "C"
								loc: SourceLocation experimental/class-private-properties/await-in-private-property-in-async/input.js 2:8-2:9 (C)
							}
							meta: JSClassHead {
								body: [
									JSClassPrivateProperty {
										leadingComments: ["0"]
										key: JSPrivateName {
											id: JSIdentifier {
												name: "p"
												loc: SourceLocation experimental/class-private-properties/await-in-private-property-in-async/input.js 4:5-4:6 (p)
											}
											loc: SourceLocation experimental/class-private-properties/await-in-private-property-in-async/input.js 4:4-4:6
										}
										meta: JSClassPropertyMeta {
											abstract: false
											optional: false
											readonly: false
											static: false
											loc: SourceLocation experimental/class-private-properties/await-in-private-property-in-async/input.js 4:4-4:6
											start: Position 4:4
										}
										value: JSAwaitExpression {
											argument: JSUnaryExpression {
												operator: "+"
												prefix: true
												argument: JSNumericLiteral {
													value: 42
													loc: SourceLocation experimental/class-private-properties/await-in-private-property-in-async/input.js 4:17-4:19
												}
												loc: SourceLocation experimental/class-private-properties/await-in-private-property-in-async/input.js 4:15-4:19
											}
											loc: SourceLocation experimental/class-private-properties/await-in-private-property-in-async/input.js 4:9-4:19
										}
										loc: SourceLocation experimental/class-private-properties/await-in-private-property-in-async/input.js 4:4-4:20
									}
								]
								loc: SourceLocation experimental/class-private-properties/await-in-private-property-in-async/input.js 2:2-5:3
							}
							loc: SourceLocation experimental/class-private-properties/await-in-private-property-in-async/input.js 2:2-5:3
						}
					]
					directives: []
					loc: SourceLocation experimental/class-private-properties/await-in-private-property-in-async/input.js 1:12-6:1
				}
				head: JSFunctionHead {
					async: true
					hasHoistedVars: false
					params: []
					loc: SourceLocation experimental/class-private-properties/await-in-private-property-in-async/input.js 1:0-1:11
				}
				loc: SourceLocation experimental/class-private-properties/await-in-private-property-in-async/input.js 1:0-6:1
			}
			loc: SourceLocation experimental/class-private-properties/await-in-private-property-in-async/input.js 1:0-6:1
		}
	]
	comments: [
		CommentLine {
			id: "0"
			value: " here await is an identifier reference"
			loc: SourceLocation experimental/class-private-properties/await-in-private-property-in-async/input.js 3:4-3:44
		}
	]
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<experimental/class-private-properties/await-in-private-property-in-async/input.js>
	loc: SourceLocation experimental/class-private-properties/await-in-private-property-in-async/input.js 1:0-7:0
}
```

### `diagnostics`

```

```