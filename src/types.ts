import { z } from 'zod';

// Enum para Loaders
export enum Loaders {
  Vanilla = 'Vanilla',
  Fabric = 'Fabric',
  Forge = 'Forge',
  Quilt = 'Quilt',
  NeoForge = 'NeoForge'
}

// Schema para Loaders
export const LoadersSchema = z.nativeEnum(Loaders);

// Enum para WindowActionResult
export const WindowActionResultSchema = z.enum([
  'MinimizeSuccess',
  'MaximizeSuccess',
  'CloseSuccess'
]);

// Enum para CubicInternalError
export const CubicInternalErrorSchema = z.enum([
  'WindowMinimizeError',
  'WindowIsNotMinimizable',
  'WindowIsNotMaximizable',
  'WindowMaximizeError',
  'WindowIsNotClosable',
  'WindowCloseError',
  'LauncherError',
  'ConfigError',
  'MinecraftInstanceError',
  'NetworkError',
  'FileError',
  'PermissionError',
  'InstanceEncodeError',
  'InvalidLoader'
]);

// Schema para Instance (usando el enum Loaders)
export const InstanceSchema = z.object({
  name: z.string(),
  loader: LoadersSchema,
  version: z.string(),
  custom_args: z.array(z.string()),
  downloaded: z.boolean()
});

// Schema para ResponseData (formato tagged enum de Rust)
export const ResponseDataSchema = z.union([
  z.object({
    MinecraftVersions: z.array(z.string())
  }),
  z.object({
    Settings: z.array(z.string())
  }),
  z.object({
    Instances: z.array(z.string())
  }),
  z.object({
    WindowAction: WindowActionResultSchema
  }),
  z.object({
    InstanceData: z.array(z.number())
  }),
  z.object({
    InstancesVec: z.array(InstanceSchema)
  })
]);

// Schema para ClientError
export const ClientErrorSchema = z.object({
  error_type: CubicInternalErrorSchema,
  error_message: z.string().optional()
});

// Schema para BackendResponse (error puede ser null)
export const BackendResponseSchema = z.object({
  success: z.boolean(),
  error: ClientErrorSchema.nullable(),
  data: ResponseDataSchema.nullable()
});

// Tipos TypeScript inferidos de los schemas
export type WindowActionResult = z.infer<typeof WindowActionResultSchema>;
export type CubicInternalError = z.infer<typeof CubicInternalErrorSchema>;
export type Instance = z.infer<typeof InstanceSchema>;
export type ResponseData = z.infer<typeof ResponseDataSchema>;
export type ClientError = z.infer<typeof ClientErrorSchema>;
export type BackendResponse = z.infer<typeof BackendResponseSchema>;

// Interfaces para uso más tradicional (alternativa a los tipos inferidos)
export interface IWindowActionResult {
  MinimizeSuccess: never;
  MaximizeSuccess: never;
  CloseSuccess: never;
}

export interface IInstance {
  name: string;
  loader: Loaders;
  version: string;
  custom_args: string[];
  downloaded: boolean;
}

export interface IClientError {
  error_type: CubicInternalError;
  error_message?: string;
}

export interface IBackendResponse {
  success: boolean;
  error?: IClientError;
  data?: ResponseData;
}

// Funciones helper para validación
export const validateBackendResponse = (data: unknown): BackendResponse => {
  return BackendResponseSchema.parse(data);
};

export const isBackendResponse = (data: unknown): data is BackendResponse => {
  return BackendResponseSchema.safeParse(data).success;
};

// Type guards específicos para cada tipo de ResponseData (formato Rust)
export const isMinecraftVersionsResponse = (data: ResponseData): data is { MinecraftVersions: string[] } => {
  return 'MinecraftVersions' in data;
};

export const isInstancesResponse = (data: ResponseData): data is { Instances: string[] } => {
  return 'Instances' in data;
};

export const isInstancesVecResponse = (data: ResponseData): data is { InstancesVec: Instance[] } => {
  return 'InstancesVec' in data;
};

export const isWindowActionResponse = (data: ResponseData): data is { WindowAction: WindowActionResult } => {
  return 'WindowAction' in data;
};

export const isInstanceDataResponse = (data: ResponseData): data is { InstanceData: number[] } => {
  return 'InstanceData' in data;
};

export const isSettingsResponse = (data: ResponseData): data is { Settings: string[] } => {
  return 'Settings' in data;
};