/*
 * The Shadow Simulator
 * See LICENSE for licensing information
 */
// clang-format off


#ifndef main_bindings_h
#define main_bindings_h

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "main/host/syscall_types.h"
#include "main/host/thread.h"

// A queue of byte chunks.
typedef struct ByteQueue ByteQueue;

// Manages the address-space for a plugin process.
typedef struct MemoryManager MemoryManager;

void bytequeue_free(ByteQueue *bq_ptr);

ByteQueue *bytequeue_new(size_t chunk_size);

size_t bytequeue_pop(ByteQueue *bq, unsigned char *dst, size_t len);

void bytequeue_push(ByteQueue *bq, const unsigned char *src, size_t len);

void memorymanager_free(MemoryManager *mm);

// Get a readable pointer to the plugin's memory via mapping, or via the thread APIs.
const void *memorymanager_getReadablePtr(MemoryManager *memory_manager,
                                         Thread *thread,
                                         PluginPtr plugin_src,
                                         uintptr_t n);

// Get a writeagble pointer to the plugin's memory via mapping, or via the thread APIs.
void *memorymanager_getWriteablePtr(MemoryManager *memory_manager,
                                    Thread *thread,
                                    PluginPtr plugin_src,
                                    uintptr_t n);

// Fully handles the `brk` syscall, keeping the "heap" mapped in our shared mem file.
SysCallReg memorymanager_handleBrk(MemoryManager *memory_manager,
                                   Thread *thread,
                                   PluginPtr plugin_src);

MemoryManager *memorymanager_new(Thread *thread);

// Notifies memorymanager that plugin is about to call execve.
void memorymanager_preExecHook(MemoryManager *memory_manager, Thread *thread);

#endif /* main_bindings_h */
