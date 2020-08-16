# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `experimental > module-attributes > invalid-spread-element-import-call`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: true
	directives: Array []
	filename: "experimental/module-attributes/invalid-spread-element-import-call/input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "module"
	syntax: Array []
	loc: Object {
		filename: "experimental/module-attributes/invalid-spread-element-import-call/input.js"
		end: Object {
			column: 0
			line: 2
		}
		start: Object {
			column: 0
			line: 1
		}
	}
	diagnostics: Array [
		Object {
			origins: Array [Object {category: "parse/js"}]
			description: Object {
				advice: Array []
				category: "parse/js"
				message: MARKUP {parts: Array [RAW_MARKUP {value: "Trailing comma is disallowed inside import(...) arguments"}]}
			}
			location: Object {
				filename: "experimental/module-attributes/invalid-spread-element-import-call/input.js"
				mtime: undefined
				sourceText: undefined
				end: Object {
					column: 20
					line: 1
				}
				start: Object {
					column: 19
					line: 1
				}
			}
		}
	]
	body: Array [
		JSExpressionStatement {
			loc: Object {
				filename: "experimental/module-attributes/invalid-spread-element-import-call/input.js"
				end: Object {
					column: 20
					line: 1
				}
				start: Object {
					column: 0
					line: 1
				}
			}
			expression: JSImportCall {
				loc: Object {
					filename: "experimental/module-attributes/invalid-spread-element-import-call/input.js"
					end: Object {
						column: 20
						line: 1
					}
					start: Object {
						column: 6
						line: 1
					}
				}
				argument: JSStringLiteral {
					value: "./foo.json"
					loc: Object {
						filename: "experimental/module-attributes/invalid-spread-element-import-call/input.js"
						end: Object {
							column: 19
							line: 1
						}
						start: Object {
							column: 7
							line: 1
						}
					}
				}
			}
		}
		JSExpressionStatement {
			loc: Object {
				filename: "experimental/module-attributes/invalid-spread-element-import-call/input.js"
				end: Object {
					column: 26
					line: 1
				}
				start: Object {
					column: 21
					line: 1
				}
			}
			expression: JSMemberExpression {
				loc: Object {
					filename: "experimental/module-attributes/invalid-spread-element-import-call/input.js"
					end: Object {
						column: 26
						line: 1
					}
					start: Object {
						column: 21
						line: 1
					}
				}
				object: JSReferenceIdentifier {
					name: "INVALID_PLACEHOLDER"
					loc: Object {
						filename: "experimental/module-attributes/invalid-spread-element-import-call/input.js"
						end: Object {
							column: 24
							line: 1
						}
						start: Object {
							column: 21
							line: 1
						}
					}
				}
				property: JSComputedMemberProperty {
					value: JSReferenceIdentifier {
						name: "INVALID_PLACEHOLDER"
						loc: Object {
							filename: "experimental/module-attributes/invalid-spread-element-import-call/input.js"
							end: Object {
								column: 26
								line: 1
							}
							start: Object {
								column: 25
								line: 1
							}
						}
					}
					loc: Object {
						filename: "experimental/module-attributes/invalid-spread-element-import-call/input.js"
						end: Object {
							column: 26
							line: 1
						}
						start: Object {
							column: 24
							line: 1
						}
					}
				}
			}
		}
		JSExpressionStatement {
			loc: Object {
				filename: "experimental/module-attributes/invalid-spread-element-import-call/input.js"
				end: Object {
					column: 28
					line: 1
				}
				start: Object {
					column: 26
					line: 1
				}
			}
			expression: JSReferenceIdentifier {
				name: "INVALID_PLACEHOLDER"
				loc: Object {
					filename: "experimental/module-attributes/invalid-spread-element-import-call/input.js"
					end: Object {
						column: 27
						line: 1
					}
					start: Object {
						column: 26
						line: 1
					}
				}
			}
		}
	]
}
```

### `diagnostics`

```

 experimental/module-attributes/invalid-spread-element-import-call/input.js:1:19 parse/js ━━━━━━━━━━

  ✖ Trailing comma is disallowed inside import(...) arguments

    import("./foo.json", ...[]);
                       ^

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```