# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2016 > simple-parameter-list > async-arrow-function-after-unary-operator`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	diagnostics: Array []
	directives: Array []
	filename: "es2016/simple-parameter-list/async-arrow-function-after-unary-operator/input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array []
	loc: Object {
		filename: "es2016/simple-parameter-list/async-arrow-function-after-unary-operator/input.js"
		end: Object {
			column: 21
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
				filename: "es2016/simple-parameter-list/async-arrow-function-after-unary-operator/input.js"
				end: Object {
					column: 21
					line: 1
				}
				start: Object {
					column: 0
					line: 1
				}
			}
			expression: JSUnaryExpression {
				operator: "delete"
				prefix: true
				loc: Object {
					filename: "es2016/simple-parameter-list/async-arrow-function-after-unary-operator/input.js"
					end: Object {
						column: 20
						line: 1
					}
					start: Object {
						column: 0
						line: 1
					}
				}
				argument: JSArrowFunctionExpression {
					loc: Object {
						filename: "es2016/simple-parameter-list/async-arrow-function-after-unary-operator/input.js"
						end: Object {
							column: 20
							line: 1
						}
						start: Object {
							column: 7
							line: 1
						}
					}
					body: JSNumericLiteral {
						value: 3
						format: undefined
						loc: Object {
							filename: "es2016/simple-parameter-list/async-arrow-function-after-unary-operator/input.js"
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
					head: JSFunctionHead {
						async: true
						hasHoistedVars: false
						params: Array []
						rest: undefined
						returnType: undefined
						thisType: undefined
						loc: Object {
							filename: "es2016/simple-parameter-list/async-arrow-function-after-unary-operator/input.js"
							end: Object {
								column: 18
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
		}
	]
}
```

### `diagnostics`

```
✔ No known problems!

```