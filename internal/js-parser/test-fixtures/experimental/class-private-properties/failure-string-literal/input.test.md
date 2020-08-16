# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `experimental > class-private-properties > failure-string-literal`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	directives: Array []
	filename: "experimental/class-private-properties/failure-string-literal/input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array []
	loc: Object {
		filename: "experimental/class-private-properties/failure-string-literal/input.js"
		end: Object {
			column: 0
			line: 4
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
				message: MARKUP {parts: Array [RAW_MARKUP {value: "Expected an identifier"}]}
			}
			location: Object {
				filename: "experimental/class-private-properties/failure-string-literal/input.js"
				mtime: undefined
				sourceText: undefined
				end: Object {
					column: 3
					line: 2
				}
				start: Object {
					column: 3
					line: 2
				}
			}
		}
	]
	body: Array [
		JSClassDeclaration {
			id: JSBindingIdentifier {
				name: "Foo"
				loc: Object {
					filename: "experimental/class-private-properties/failure-string-literal/input.js"
					identifierName: "Foo"
					end: Object {
						column: 9
						line: 1
					}
					start: Object {
						column: 6
						line: 1
					}
				}
			}
			loc: Object {
				filename: "experimental/class-private-properties/failure-string-literal/input.js"
				end: Object {
					column: 1
					line: 3
				}
				start: Object {
					column: 0
					line: 1
				}
			}
			meta: JSClassHead {
				implements: undefined
				superClass: undefined
				superTypeParameters: undefined
				typeParameters: undefined
				loc: Object {
					filename: "experimental/class-private-properties/failure-string-literal/input.js"
					end: Object {
						column: 1
						line: 3
					}
					start: Object {
						column: 0
						line: 1
					}
				}
				body: Array [
					JSClassPrivateProperty {
						key: JSPrivateName {
							id: JSIdentifier {
								name: ""
								loc: Object {
									filename: "experimental/class-private-properties/failure-string-literal/input.js"
									identifierName: ""
									end: Object {
										column: 6
										line: 2
									}
									start: Object {
										column: 3
										line: 2
									}
								}
							}
							loc: Object {
								filename: "experimental/class-private-properties/failure-string-literal/input.js"
								end: Object {
									column: 6
									line: 2
								}
								start: Object {
									column: 2
									line: 2
								}
							}
						}
						value: JSReferenceIdentifier {
							name: "x"
							loc: Object {
								filename: "experimental/class-private-properties/failure-string-literal/input.js"
								identifierName: "x"
								end: Object {
									column: 10
									line: 2
								}
								start: Object {
									column: 9
									line: 2
								}
							}
						}
						typeAnnotation: undefined
						loc: Object {
							filename: "experimental/class-private-properties/failure-string-literal/input.js"
							end: Object {
								column: 10
								line: 2
							}
							start: Object {
								column: 2
								line: 2
							}
						}
						meta: JSClassPropertyMeta {
							abstract: false
							accessibility: undefined
							optional: false
							readonly: false
							static: false
							typeAnnotation: undefined
							start: Object {
								column: 2
								line: 2
							}
							loc: Object {
								filename: "experimental/class-private-properties/failure-string-literal/input.js"
								end: Object {
									column: 6
									line: 2
								}
								start: Object {
									column: 2
									line: 2
								}
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

 experimental/class-private-properties/failure-string-literal/input.js:2:3 parse/js ━━━━━━━━━━━━━━━━

  ✖ Expected an identifier

    1 │ class Foo {
  > 2 │   #"p" = x
      │    ^
    3 │ }

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```