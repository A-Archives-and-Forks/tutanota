import o from "@tutao/otest"
import { getAttributeId, resolveTypeReference } from "../../../../src/common/api/common/EntityFunctions"
import { when } from "testdouble"
import { typeModels } from "../../../../src/common/api/entities/base/TypeModels"
import { TypeRef } from "@tutao/tutanota-utils"

o.spec("EntityFunctionsTest", function () {
	o.spec("can get attribute id lazily", async () => {
		let resolveWasCalled = false
		when(resolveTypeReference).thenResolve(() => {
			resolveWasCalled = true
		})
		when(typeModels).thenResolve({
			"0": {
				name: "PersistenceResourcePostReturn",
				since: 1,
				type: "DATA_TRANSFER_TYPE",
				id: 0,
				rootId: "BGJhc2UAAA",
				versioned: false,
				encrypted: false,
				values: {
					"10": {
						final: false,
						name: "name_10",
						id: 1,
						since: 1,
						type: "Number",
						cardinality: "One",
						encrypted: false,
					},
				},
			},
		})
		o(await getAttributeId(new TypeRef("base", 0), "name_10")).equals(10)
		o(resolveWasCalled).equals(true)
	})
})
