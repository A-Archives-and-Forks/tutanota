import { create, Stripped, StrippedEntity } from "../../common/utils/EntityUtils.js"
import { TypeRef } from "@tutao/tutanota-utils"
import { typeModels } from "./TypeModels.js"


export const ReadCounterDataTypeRef: TypeRef<ReadCounterData> = new TypeRef("monitor", 12)

export function createReadCounterData(values: StrippedEntity<ReadCounterData>): ReadCounterData {
	return Object.assign(create(typeModels[ReadCounterDataTypeRef.typeId], ReadCounterDataTypeRef), values)
}

export type ReadCounterData = {
	_type: TypeRef<ReadCounterData>;

	rowName: string;
	columnName: null | Id;
	counterType: NumberString;
	_format: NumberString;
}
export const ReadCounterReturnTypeRef: TypeRef<ReadCounterReturn> = new TypeRef("monitor", 16)

export function createReadCounterReturn(values: StrippedEntity<ReadCounterReturn>): ReadCounterReturn {
	return Object.assign(create(typeModels[ReadCounterReturnTypeRef.typeId], ReadCounterReturnTypeRef), values)
}

export type ReadCounterReturn = {
	_type: TypeRef<ReadCounterReturn>;

	value: null | NumberString;
	_format: NumberString;

	counterValues: CounterValue[];
}
export const WriteCounterDataTypeRef: TypeRef<WriteCounterData> = new TypeRef("monitor", 49)

export function createWriteCounterData(values: StrippedEntity<WriteCounterData>): WriteCounterData {
	return Object.assign(create(typeModels[WriteCounterDataTypeRef.typeId], WriteCounterDataTypeRef), values)
}

export type WriteCounterData = {
	_type: TypeRef<WriteCounterData>;

	row: string;
	column: Id;
	value: NumberString;
	counterType: null | NumberString;
	_format: NumberString;
}
export const ApprovalMailTypeRef: TypeRef<ApprovalMail> = new TypeRef("monitor", 221)

export function createApprovalMail(values: StrippedEntity<ApprovalMail>): ApprovalMail {
	return Object.assign(create(typeModels[ApprovalMailTypeRef.typeId], ApprovalMailTypeRef), values)
}

export type ApprovalMail = {
	_type: TypeRef<ApprovalMail>;

	range: null | string;
	date: null | Date;
	text: string;
	_format: NumberString;
	_id: IdTuple;
	_ownerGroup: null | Id;
	_permissions: Id;

	customer: null | Id;
}
export const CounterValueTypeRef: TypeRef<CounterValue> = new TypeRef("monitor", 300)

export function createCounterValue(values: StrippedEntity<CounterValue>): CounterValue {
	return Object.assign(create(typeModels[CounterValueTypeRef.typeId], CounterValueTypeRef), values)
}

export type CounterValue = {
	_type: TypeRef<CounterValue>;

	counterId: Id;
	value: NumberString;
	_id: Id;
}
export const ErrorReportFileTypeRef: TypeRef<ErrorReportFile> = new TypeRef("monitor", 305)

export function createErrorReportFile(values: StrippedEntity<ErrorReportFile>): ErrorReportFile {
	return Object.assign(create(typeModels[ErrorReportFileTypeRef.typeId], ErrorReportFileTypeRef), values)
}

export type ErrorReportFile = {
	_type: TypeRef<ErrorReportFile>;

	name: string;
	content: string;
	_id: Id;
}
export const ErrorReportDataTypeRef: TypeRef<ErrorReportData> = new TypeRef("monitor", 316)

export function createErrorReportData(values: StrippedEntity<ErrorReportData>): ErrorReportData {
	return Object.assign(create(typeModels[ErrorReportDataTypeRef.typeId], ErrorReportDataTypeRef), values)
}

export type ErrorReportData = {
	_type: TypeRef<ErrorReportData>;

	time: Date;
	appVersion: string;
	clientType: NumberString;
	userId: null | string;
	errorClass: string;
	errorMessage: null | string;
	stackTrace: string;
	userMessage: null | string;
	additionalInfo: string;
	_id: Id;
}
export const ReportErrorInTypeRef: TypeRef<ReportErrorIn> = new TypeRef("monitor", 335)

export function createReportErrorIn(values: StrippedEntity<ReportErrorIn>): ReportErrorIn {
	return Object.assign(create(typeModels[ReportErrorInTypeRef.typeId], ReportErrorInTypeRef), values)
}

export type ReportErrorIn = {
	_type: TypeRef<ReportErrorIn>;

	_format: NumberString;

	data: ErrorReportData;
	files: ErrorReportFile[];
}
