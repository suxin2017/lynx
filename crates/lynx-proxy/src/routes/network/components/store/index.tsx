import { configureStore } from '@reduxjs/toolkit';
import { requestTreeReducer } from './requestTreeStore';
import { requestTableReducer } from './requestTableStore';
import { selectRequestReducer } from './selectRequestStore';

export const store = configureStore({
  reducer: {
    requestTable: requestTableReducer,
    requestTree: requestTreeReducer,
    selectRequest: selectRequestReducer,
  },
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware({ serializableCheck: false }),
});

// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>;
// Inferred type: {posts: PostsState, comments: CommentsState, users: UsersState}
export type AppDispatch = typeof store.dispatch;
