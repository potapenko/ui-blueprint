#if UIBLUEPRINT_RUNTIME_DEBUG
#include "RuntimeUpdateScope.h"
#include <dlfcn.h>
#include <pthread.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>
#include <TargetConditionals.h>

// These concrete exported Swift signatures were checked against Xcode27.2's
// SwiftUICore.tbd and macOS27.0.1 code. swift_context supplies class metadata
// for static getters, and instance self for methods, in the Swift ABI context
// register. Neither an ordinary C argument nor an omitted context is correct.
typedef void *(*GroupGetter)(const void *metatype __attribute__((swift_context))) __attribute__((swiftcall));
typedef void (*GroupMethod)(void *self __attribute__((swift_context))) __attribute__((swiftcall));
typedef bool (*GroupPredicate)(void *self __attribute__((swift_context))) __attribute__((swiftcall));
typedef void (*ReleaseObject)(void *);

static GroupGetter main_group, current_group;
static const void *group_metadata;
static GroupMethod begin_group, end_group;
static GroupPredicate is_main;
static ReleaseObject release_object;
static pthread_once_t symbols_once = PTHREAD_ONCE_INIT;

static void *system_symbol(const char *name, const char *image_prefix) {
    void *value = dlsym(RTLD_DEFAULT, name);
    Dl_info info = {0};
    if (!value || !dladdr(value, &info) || !info.dli_fname) return NULL;
    const char *path = info.dli_fname;
#if TARGET_OS_SIMULATOR
    // Simulator system images may be rooted under the selected runtime bundle.
    // Only strip that exact dyld-provided root, never search an arbitrary path.
    const char *root = getenv("DYLD_ROOT_PATH");
    if (root && root[0] && strncmp(path, root, strlen(root)) == 0) path += strlen(root);
#endif
    if (strncmp(path, image_prefix, strlen(image_prefix)) != 0) return NULL;
    return value;
}

static void load_symbols(void) {
    const char *image = "/System/Library/Frameworks/SwiftUICore.framework/";
    group_metadata = system_symbol("$s7SwiftUI11UpdateGroupCN", image);
    main_group = (GroupGetter)system_symbol("$s7SwiftUI11UpdateGroupC4mainACvgZ", image);
    current_group = (GroupGetter)system_symbol("$s7SwiftUI11UpdateGroupC7currentACSgvgZ", image);
    begin_group = (GroupMethod)system_symbol("$s7SwiftUI11UpdateGroupC5beginyyF", image);
    end_group = (GroupMethod)system_symbol("$s7SwiftUI11UpdateGroupC3endyyF", image);
    is_main = (GroupPredicate)system_symbol("$s7SwiftUI11UpdateGroupC6isMainSbvg", image);
    release_object = (ReleaseObject)system_symbol("swift_release", "/usr/lib/swift/libswiftCore.dylib");
}

int UIBRuntimeUpdateCurrent(void) {
    pthread_once(&symbols_once, load_symbols);
    if (!group_metadata || !current_group || !release_object || !pthread_main_np()) return -1;
    // Exported Swift object getters return an owned (+1) reference.
    void *value = current_group(group_metadata);
    if (value) release_object(value);
    return value != NULL;
}

void *UIBRuntimeUpdateBegin(void) {
    pthread_once(&symbols_once, load_symbols);
    if (!group_metadata || !main_group || !current_group || !begin_group || !end_group || !is_main ||
        !release_object || !pthread_main_np()) return NULL;
    void *previous = current_group(group_metadata);
    bool compatible = !previous || is_main(previous);
    if (previous) release_object(previous);
    if (!compatible) return NULL;
    void *group = main_group(group_metadata);
    if (!group) return NULL;
    if (!is_main(group)) { release_object(group); return NULL; }
    begin_group(group);
    void *current = current_group(group_metadata);
    bool entered = current == group;
    if (current) release_object(current);
    if (!entered) {
        end_group(group);
        release_object(group);
        return NULL;
    }
    return group;
}

void UIBRuntimeUpdateEnd(void *token) {
    if (!token) return;
    end_group(token);
    release_object(token);
}
#endif
