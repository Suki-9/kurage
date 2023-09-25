# Misskey Endpoints

## admin

### accounts
#### create
path : "admin/accounts/create"<br/>
```ts
req: {
	username: string;
	password: string;
};
res: UserLite;
```

#### delete
path : "admin/accounts/delete"<br/>
Credentials required !
```ts
req: {
	userId: string;
};
res: null;
```

### ad
#### create
path : "admin/ad/create"<br/>
Credentials required !
```ts
req: {
  url: string;
  memo: string;
  place: string;
  priority: string;
  ratio: number;
  expiresAt: number;
  imageUrl: string;
};
res: null;
```

#### delete
path : "admin/ad/delete"<br/>
Credentials required !
```ts
req: {
  id: string;
};
res: null;
```

#### list
path : "admin/ad/list"<br/>
Credentials required !
```ts
req: {
  limit: number = 10;
  sinceId: string;
  untilId: string;
}
res: null;
```

#### update
path : "admin/ad/update"<br/>
Credentials required !
```ts
req: {
  url: string;
  memo: string;
  place: string;
  priority: string;
  ratio: number;
  expiresAt: number;
  imageUrl: string;
};
res: null;
```

### announcements
#### create
path : "admin/announcement/"<br/>
Credentials required !
```ts
```
#### delete
path : "admin/announcement/delete"<br/>
```ts
req: {
  title: string;
  text: string;
  imageUrl: string | null;
}
res: {
  id: string;
  createdAt: string;
  updatedAt: string | null;
  title: string;
  text: string;
  imageUrl: string | null;
}
```
#### list
path : "admin/announcement/list"<br/>
```ts
req: {
  limit: number;
  sinceId: string;
  untilId: string;
}
req: {
  id: string;
  createdAt: string;
  updatedAt: string | null;
  text: string;
  title: string;
  imageUrl: string | null;
  reads: number;
}
```
#### update
path : "admin/announcement/update"<br/>
```ts
req: {
  id: string;
  title: string;
  text: string;
  imageUrl: string | null;
}
res: null;
```