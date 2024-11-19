#ifndef MUTEX_H
#define MUTEX_H

typedef struct FFIMutex FFIMutex;

FFIMutex* mutex_create();
void* mutex_lock(FFIMutex* mutex);
void mutex_unlock(void* guard);
void mutex_destroy(FFIMutex* mutex);

#endif