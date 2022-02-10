# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `experimental > class-private-properties > asi-failure-generator`

### `ast`

```javascript
JSRoot {
	body: [
		JSClassDeclaration {
			id: JSBindingIdentifier {
				name: "Foo"
				loc: SourceLocation experimental/class-private-properties/asi-failure-generator/input.js 1:6-1:9 (Foo)
			}
			meta: JSClassHead {
				body: [
					JSClassPrivateProperty {
						key: JSPrivateName {
							id: JSIdentifier {
								name: "p"
								loc: SourceLocation experimental/class-private-properties/asi-failure-generator/input.js 2:3-2:4 (p)
							}
							loc: SourceLocation experimental/class-private-properties/asi-failure-generator/input.js 2:2-2:4
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: false
							loc: SourceLocation experimental/class-private-properties/asi-failure-generator/input.js 2:2-2:4
							start: Position 2:2
						}
						value: JSBinaryExpression {
							operator: "*"
							left: JSReferenceIdentifier {
								name: "x"
								loc: SourceLocation experimental/class-private-properties/asi-failure-generator/input.js 2:7-2:8 (x)
							}
							right: JSCallExpression {
								arguments: []
								callee: JSReferenceIdentifier {
									name: "m"
									loc: SourceLocation experimental/class-private-properties/asi-failure-generator/input.js 3:3-3:4 (m)
								}
								loc: SourceLocation experimental/class-private-properties/asi-failure-generator/input.js 3:3-3:7
							}
							loc: SourceLocation experimental/class-private-properties/asi-failure-generator/input.js 2:7-3:7
						}
						loc: SourceLocation experimental/class-private-properties/asi-failure-generator/input.js 2:2-3:7
					}
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: ""
								loc: SourceLocation experimental/class-private-properties/asi-failure-generator/input.js 3:8-3:9 ()
							}
							loc: SourceLocation experimental/class-private-properties/asi-failure-generator/input.js 3:8-3:9
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: false
							loc: SourceLocation experimental/class-private-properties/asi-failure-generator/input.js 3:8-3:9
							start: Position 3:8
						}
						loc: SourceLocation experimental/class-private-properties/asi-failure-generator/input.js 3:8-3:9
					}
				]
				loc: SourceLocation experimental/class-private-properties/asi-failure-generator/input.js 1:0-3:10
			}
			loc: SourceLocation experimental/class-private-properties/asi-failure-generator/input.js 1:0-3:10
		}
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "INVALID_PLACEHOLDER"
				loc: SourceLocation experimental/class-private-properties/asi-failure-generator/input.js 4:0-4:1
			}
			loc: SourceLocation experimental/class-private-properties/asi-failure-generator/input.js 4:0-4:1
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
				path: UIDPath<experimental/class-private-properties/asi-failure-generator/input.js>
				end: Position 3:7
				start: Position 3:8
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<experimental/class-private-properties/asi-failure-generator/input.js>
	loc: SourceLocation experimental/class-private-properties/asi-failure-generator/input.js 1:0-5:0
}
```

### `diagnostics`

```

 experimental/class-private-properties/asi-failure-generator/input.js:3:8 parse(js) ━━━━━━━━━━━━━━━━

  ✖ Expected a semicolon or a line terminator

    1 │ class Foo {
    2 │   #p = x
  > 3 │   *m () {}
      │         ^
    4 │ }


```