import { create, Stripped, StrippedEntity } from "../../common/utils/EntityUtils.js"
import { TypeRef } from "@tutao/tutanota-utils"
import { typeModels } from "./TypeModels.js"


export const CustomerAccountPostingTypeRef: TypeRef<CustomerAccountPosting> = new TypeRef("accounting", 79)

export function createCustomerAccountPosting(values: StrippedEntity<CustomerAccountPosting>): CustomerAccountPosting {
	return Object.assign(create(typeModels[CustomerAccountPostingTypeRef.typeId], CustomerAccountPostingTypeRef), values)
}

export type CustomerAccountPosting = {
	_type: TypeRef<CustomerAccountPosting>;

	type: NumberString;
	valueDate: Date;
	invoiceNumber: null | string;
	amount: NumberString;
	_id: Id;
}
export const CustomerAccountReturnTypeRef: TypeRef<CustomerAccountReturn> = new TypeRef("accounting", 86)

export function createCustomerAccountReturn(values: StrippedEntity<CustomerAccountReturn>): CustomerAccountReturn {
	return Object.assign(create(typeModels[CustomerAccountReturnTypeRef.typeId], CustomerAccountReturnTypeRef), values)
}

export type CustomerAccountReturn = {
	_type: TypeRef<CustomerAccountReturn>;
	_errors: Object;

	outstandingBookingsPrice: NumberString;
	balance: NumberString;
	_format: NumberString;
	_ownerGroup: null | Id;
	_ownerPublicEncSessionKey: null | Uint8Array;
	_publicCryptoProtocolVersion: null | NumberString;

	postings: CustomerAccountPosting[];
}
