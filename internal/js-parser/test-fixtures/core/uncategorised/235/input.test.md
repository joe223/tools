# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 235`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	diagnostics: Array []
	directives: Array []
	filename: "core/uncategorised/235/input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array []
	loc: Object {
		filename: "core/uncategorised/235/input.js"
		end: Object {
			column: 30
			line: 1
		}
		start: Object {
			column: 0
			line: 1
		}
	}
	body: Array [
		JSBlockStatement {
			directives: Array []
			loc: Object {
				filename: "core/uncategorised/235/input.js"
				end: Object {
					column: 30
					line: 1
				}
				start: Object {
					column: 0
					line: 1
				}
			}
			body: Array [
				JSDoWhileStatement {
					loc: Object {
						filename: "core/uncategorised/235/input.js"
						end: Object {
							column: 23
							line: 1
						}
						start: Object {
							column: 2
							line: 1
						}
					}
					test: JSBooleanLiteral {
						value: false
						loc: Object {
							filename: "core/uncategorised/235/input.js"
							end: Object {
								column: 21
								line: 1
							}
							start: Object {
								column: 16
								line: 1
							}
						}
					}
					body: JSBlockStatement {
						body: Array []
						directives: Array []
						loc: Object {
							filename: "core/uncategorised/235/input.js"
							end: Object {
								column: 8
								line: 1
							}
							start: Object {
								column: 5
								line: 1
							}
						}
					}
				}
				JSExpressionStatement {
					loc: Object {
						filename: "core/uncategorised/235/input.js"
						end: Object {
							column: 28
							line: 1
						}
						start: Object {
							column: 23
							line: 1
						}
					}
					expression: JSBooleanLiteral {
						value: false
						loc: Object {
							filename: "core/uncategorised/235/input.js"
							end: Object {
								column: 28
								line: 1
							}
							start: Object {
								column: 23
								line: 1
							}
						}
					}
				}
			]
		}
	]
}
```

### `diagnostics`

```
✔ No known problems!

```