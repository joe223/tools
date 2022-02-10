# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `comments > basic > switch-fallthrough-comment-in-function`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "bar"
				loc: SourceLocation comments/basic/switch-fallthrough-comment-in-function/input.js 1:9-1:12 (bar)
			}
			body: JSBlockStatement {
				body: [
					JSSwitchStatement {
						cases: [
							JSSwitchCase {
								consequent: []
								leadingComments: ["0"]
								trailingComments: ["1"]
								test: JSNumericLiteral {
									value: 1
									loc: SourceLocation comments/basic/switch-fallthrough-comment-in-function/input.js 4:13-4:14
								}
								loc: SourceLocation comments/basic/switch-fallthrough-comment-in-function/input.js 4:8-4:15
							}
							JSSwitchCase {
								consequent: [
									JSExpressionStatement {
										expression: JSCallExpression {
											arguments: []
											callee: JSReferenceIdentifier {
												name: "doIt"
												loc: SourceLocation comments/basic/switch-fallthrough-comment-in-function/input.js 7:12-7:16 (doIt)
											}
											loc: SourceLocation comments/basic/switch-fallthrough-comment-in-function/input.js 7:12-7:18
										}
										loc: SourceLocation comments/basic/switch-fallthrough-comment-in-function/input.js 7:12-7:19
									}
								]
								leadingComments: ["1"]
								test: JSNumericLiteral {
									value: 2
									loc: SourceLocation comments/basic/switch-fallthrough-comment-in-function/input.js 6:13-6:14
								}
								loc: SourceLocation comments/basic/switch-fallthrough-comment-in-function/input.js 6:8-7:19
							}
						]
						discriminant: JSReferenceIdentifier {
							name: "foo"
							loc: SourceLocation comments/basic/switch-fallthrough-comment-in-function/input.js 2:11-2:14 (foo)
						}
						loc: SourceLocation comments/basic/switch-fallthrough-comment-in-function/input.js 2:4-8:5
					}
				]
				directives: []
				loc: SourceLocation comments/basic/switch-fallthrough-comment-in-function/input.js 1:18-9:1
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: false
				params: [
					JSBindingIdentifier {
						name: "foo"
						meta: JSPatternMeta {
							loc: SourceLocation comments/basic/switch-fallthrough-comment-in-function/input.js 1:13-1:16
						}
						loc: SourceLocation comments/basic/switch-fallthrough-comment-in-function/input.js 1:13-1:16 (foo)
					}
				]
				loc: SourceLocation comments/basic/switch-fallthrough-comment-in-function/input.js 1:12-1:17
			}
			loc: SourceLocation comments/basic/switch-fallthrough-comment-in-function/input.js 1:0-9:1
		}
	]
	comments: [
		CommentLine {
			id: "0"
			value: " foo"
			loc: SourceLocation comments/basic/switch-fallthrough-comment-in-function/input.js 3:8-3:14
		}
		CommentLine {
			id: "1"
			value: " falls through"
			loc: SourceLocation comments/basic/switch-fallthrough-comment-in-function/input.js 5:12-5:28
		}
	]
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<comments/basic/switch-fallthrough-comment-in-function/input.js>
	loc: SourceLocation comments/basic/switch-fallthrough-comment-in-function/input.js 1:0-10:0
}
```

### `diagnostics`

```

```