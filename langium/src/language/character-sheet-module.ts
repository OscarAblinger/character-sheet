import type { DefaultSharedModuleContext, LangiumServices, LangiumSharedServices, Module, PartialLangiumServices } from 'langium';
import { createDefaultModule, createDefaultSharedModule, inject } from 'langium';
import { CharacterSheetGeneratedModule, CharacterSheetGeneratedSharedModule } from './generated/module.js';
import { CharacterSheetValidator, registerValidationChecks } from './character-sheet-validator.js';

/**
 * Declaration of custom services - add your own service classes here.
 */
export type CharacterSheetAddedServices = {
    validation: {
        CharacterSheetValidator: CharacterSheetValidator
    }
}

/**
 * Union of Langium default services and your custom services - use this as constructor parameter
 * of custom service classes.
 */
export type CharacterSheetServices = LangiumServices & CharacterSheetAddedServices

/**
 * Dependency injection module that overrides Langium default services and contributes the
 * declared custom services. The Langium defaults can be partially specified to override only
 * selected services, while the custom services must be fully specified.
 */
export const CharacterSheetModule: Module<CharacterSheetServices, PartialLangiumServices & CharacterSheetAddedServices> = {
    validation: {
        CharacterSheetValidator: () => new CharacterSheetValidator()
    }
};

/**
 * Create the full set of services required by Langium.
 *
 * First inject the shared services by merging two modules:
 *  - Langium default shared services
 *  - Services generated by langium-cli
 *
 * Then inject the language-specific services by merging three modules:
 *  - Langium default language-specific services
 *  - Services generated by langium-cli
 *  - Services specified in this file
 *
 * @param context Optional module context with the LSP connection
 * @returns An object wrapping the shared services and the language-specific services
 */
export function createCharacterSheetServices(context: DefaultSharedModuleContext): {
    shared: LangiumSharedServices,
    CharacterSheet: CharacterSheetServices
} {
    const shared = inject(
        createDefaultSharedModule(context),
        CharacterSheetGeneratedSharedModule
    );
    const CharacterSheet = inject(
        createDefaultModule({ shared }),
        CharacterSheetGeneratedModule,
        CharacterSheetModule
    );
    shared.ServiceRegistry.register(CharacterSheet);
    registerValidationChecks(CharacterSheet);
    return { shared, CharacterSheet };
}
