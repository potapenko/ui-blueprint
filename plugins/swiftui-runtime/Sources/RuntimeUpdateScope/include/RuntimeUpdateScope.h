#pragma once

// Internal Debug-only bridge. Tokens own one retained SwiftUI UpdateGroup.
void *UIBRuntimeUpdateBegin(void);
void UIBRuntimeUpdateEnd(void *token);
int UIBRuntimeUpdateCurrent(void);
