import { z } from "zod/v4";

// Base validation schemas
const hexColor = z.string().regex(/^#[0-9A-Fa-f]{6}$/, {
  message: "Debe ser un color hexadecimal válido con formato #RRGGBB",
});

// Component schemas
const metaSchema = z.object({
  name: z.string(),
  author: z.string(),
  tone: z.enum(["dark", "light"]),
});

const textSchema = z.object({
  primary: hexColor,
  secondary: hexColor,
  disabled: hexColor,
});

const accentSchema = z.object({
  base: hexColor,
  hover: hexColor,
  active: hexColor,
});

const borderSchema = z.object({
  default: hexColor,
  subtle: hexColor,
});

const stateSchema = z.object({
  error: hexColor,
  warning: hexColor,
  success: hexColor,
  info: hexColor,
});

// Main theme schema
const themeSchema = z.object({
  background: hexColor,
  surface: hexColor,
  text: textSchema,
  logo: hexColor,
  accent: accentSchema,
  border: borderSchema,
  state: stateSchema,
  button: accentSchema,
});

// Full theme schema
export const fullThemeSchema = z.object({
  meta: metaSchema,
  ...themeSchema.shape,
});

// TypeScript type inference
export type HexColor = z.infer<typeof hexColor>;
export type Meta = z.infer<typeof metaSchema>;
export type Text = z.infer<typeof textSchema>;
export type Accent = z.infer<typeof accentSchema>;
export type Border = z.infer<typeof borderSchema>;
export type State = z.infer<typeof stateSchema>;
export type Theme = z.infer<typeof themeSchema>;
export type FullTheme = z.infer<typeof fullThemeSchema>;

// Theme enumeration
export enum Themes {
  moka = "moka",
  moka_white = "moka_white",
}
