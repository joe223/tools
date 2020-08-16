# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > uncategorised > 174`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	diagnostics: Array []
	directives: Array []
	filename: "es2015/uncategorised/174/input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array []
	loc: Object {
		filename: "es2015/uncategorised/174/input.js"
		end: Object {
			column: 22
			line: 1
		}
		start: Object {
			column: 0
			line: 1
		}
	}
	body: Array [
		JSExpressionStatement {
			loc: Object {
				filename: "es2015/uncategorised/174/input.js"
				end: Object {
					column: 22
					line: 1
				}
				start: Object {
					column: 0
					line: 1
				}
			}
			expression: JSObjectExpression {
				loc: Object {
					filename: "es2015/uncategorised/174/input.js"
					end: Object {
						column: 21
						line: 1
					}
					start: Object {
						column: 1
						line: 1
					}
				}
				properties: Array [
					JSObjectMethod {
						kind: "method"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "x"
								loc: Object {
									filename: "es2015/uncategorised/174/input.js"
									identifierName: "x"
									end: Object {
										column: 4
										line: 1
									}
									start: Object {
										column: 3
										line: 1
									}
								}
							}
							loc: Object {
								filename: "es2015/uncategorised/174/input.js"
								end: Object {
									column: 4
									line: 1
								}
								start: Object {
									column: 3
									line: 1
								}
							}
						}
						loc: Object {
							filename: "es2015/uncategorised/174/input.js"
							end: Object {
								column: 19
								line: 1
							}
							start: Object {
								column: 3
								line: 1
							}
						}
						body: JSBlockStatement {
							body: Array []
							directives: Array []
							loc: Object {
								filename: "es2015/uncategorised/174/input.js"
								end: Object {
									column: 19
									line: 1
								}
								start: Object {
									column: 17
									line: 1
								}
							}
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: Array []
							returnType: undefined
							thisType: undefined
							typeParameters: undefined
							loc: Object {
								filename: "es2015/uncategorised/174/input.js"
								end: Object {
									column: 17
									line: 1
								}
								start: Object {
									column: 4
									line: 1
								}
							}
							rest: JSBindingArrayPattern {
								rest: undefined
								loc: Object {
									filename: "es2015/uncategorised/174/input.js"
									end: Object {
										column: 16
										line: 1
									}
									start: Object {
										column: 8
										line: 1
									}
								}
								meta: JSPatternMeta {
									optional: undefined
									typeAnnotation: undefined
									loc: Object {
										filename: "es2015/uncategorised/174/input.js"
										end: Object {
											column: 16
											line: 1
										}
										start: Object {
											column: 8
											line: 1
										}
									}
								}
								elements: Array [
									JSBindingIdentifier {
										name: "a"
										loc: Object {
											filename: "es2015/uncategorised/174/input.js"
											identifierName: "a"
											end: Object {
												column: 11
												line: 1
											}
											start: Object {
												column: 10
												line: 1
											}
										}
										meta: JSPatternMeta {
											optional: undefined
											typeAnnotation: undefined
											loc: Object {
												filename: "es2015/uncategorised/174/input.js"
												end: Object {
													column: 11
													line: 1
												}
												start: Object {
													column: 10
													line: 1
												}
											}
										}
									}
									JSBindingIdentifier {
										name: "b"
										loc: Object {
											filename: "es2015/uncategorised/174/input.js"
											identifierName: "b"
											end: Object {
												column: 14
												line: 1
											}
											start: Object {
												column: 13
												line: 1
											}
										}
										meta: JSPatternMeta {
											optional: undefined
											typeAnnotation: undefined
											loc: Object {
												filename: "es2015/uncategorised/174/input.js"
												end: Object {
													column: 14
													line: 1
												}
												start: Object {
													column: 13
													line: 1
												}
											}
										}
									}
								]
							}
						}
					}
				]
			}
		}
	]
}
```

### `diagnostics`

```
✔ No known problems!

```