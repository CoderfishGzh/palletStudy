#![cfg_attr(not(feature = "std"), no_std)]


/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;
pub use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_debug_derive::RuntimeDebug;
use frame_support::inherent::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// struct example 
// 需要加宏 并且为该结构体实现构造函数
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct Order {
    pub name : u32,
}

impl Order {
    pub fn new(name : u32) -> Self{
        Self {
            name
        }
    }
}

// what user buy
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct UserProduct {
	pub productid : u32,
    pub productnums :u32,
} 

impl UserProduct {
    pub fn new(id : u32, nums : u32) -> Self {
        Self {
            productid : id,
            productnums : nums,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct User {
    pub products : Vec<UserProduct>,
    pub totalprice : u32,
    pub totalnums : u32,
}

impl User {
    pub fn new() -> Self {
        Self{
            products : Vec::new(),
            totalprice : 0,
            totalnums : 0,
        }
    }
}

#[frame_support::pallet] 
pub mod pallet {
    use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, Order>;

	// gzh add 
    //购物车商品数量,不同id，对应不同的购物车
    #[pallet::storage]
    #[pallet::getter(fn totalnums)]
    pub type TotalNums<T : Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32>;

    //每件商品对应的价格
    #[pallet::storage]
    #[pallet::getter(fn productprice)]
    pub type ProductPrice<T> = StorageMap<_, Blake2_128Concat, u32, u32>;

    //用户自己的购物车 已经添加的商品及商品数量
    #[pallet::storage]
    #[pallet::getter(fn user)]
    pub type WhatUserBuy<T : Config> = StorageMap<_, Blake2_128Concat, T::AccountId, User>;

    //购物车总价格,不同id，对应不同的购物车
    #[pallet::storage]
    #[pallet::getter(fn totalprice)]
    pub type TotalPrice<T : Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32>;

    //购物车各个商品的数量，两个key，一个是用户id，一个是商品id，value是数量
    #[pallet::storage]
    #[pallet::getter(fn products)]
    pub type Products<T : Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, u32, u32>;




	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),


		//gzh add 
		// user id, product id
        Addproduct(T::AccountId, u32),
        // user id, product id
        Removeproduct(T::AccountId, u32),
        //商品id， 商品数量
        Cannotremove(u32,u32),
        //商人添加商品 用户id，商品id，商品价格
        Bm_add_product(T::AccountId, u32, u32),
        //找不到商品id
        CanNotFindGoods(u32),
        
        Nothisuser(T::AccountId),

        NothisGoods(u32),

        Line223(),
        Line231(),
        Line242(),

        Line290(),
        Line309(),
        Line315(),

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

            let order = Order::new(something);

			// Update storage.
			<Something<T>>::put(order);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())

		}


        //商家添加货物
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn bm_add_product(origin: OriginFor<T>, products : u32, price : u32) -> DispatchResult {
            
            //如果商品 和 价格 都为 0 则报错
            ensure!(price != 0 || products != 0, DispatchError::NoProviders);
            
            let who = ensure_signed(origin)?;

            <ProductPrice<T>>::insert(products, price);
          
            Self::deposit_event(Event::Bm_add_product(who, products, price));

            Ok(())
        }

		//gzh add 
		//客户账号添加商品
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn addproduct(origin: OriginFor<T>, products : u32) -> DispatchResult {

            let who = ensure_signed(origin)?;

            //1.判断该商品是否在  productprice上面
            let goods = <ProductPrice<T>>::try_get(products);
            match goods {
                Ok(p) => {
                    // 找到该货物
                    //看该用户是否存在 WhatUserBuy
                    let tuser = <WhatUserBuy<T>>::try_get(&who);
                    match tuser {
                        Ok(mut t) => {
                            //用户存在
                            for mut i in &mut t.products {
                                //商品存在 修改 商品数量
                                if i.productid == products {
                                    let temp = &i.productnums;
                                    i.productnums = temp + 1;
                                    Self::deposit_event(Event::Addproduct(who.clone(), products));
                                    Self::deposit_event(Event::Line223());
                                    return Ok(());
                                }
                            }
                            //商品不存在 直接插入商品
                            let newUserProduct = UserProduct::new(products, p);
                            t.products.push(newUserProduct);
                            t.totalnums = t.totalnums + 1;
                            t.totalprice = t.totalprice + p;
                            Self::deposit_event(Event::Addproduct(who.clone(), products));
                            Self::deposit_event(Event::Line231());
                            return Ok(());
                        }

                        _ => {
                            //不存在直接添加
                            let mut newuser = User::new();
                            let newUserProduct = UserProduct::new(products, p);
                            newuser.products.push(newUserProduct);
                            newuser.totalnums = 1;
                            newuser.totalprice = p;
                            <WhatUserBuy<T>>::insert(who.clone(), newuser);
                            Self::deposit_event(Event::Addproduct(who.clone(), products));
                            Self::deposit_event(Event::Line242());
                            return Ok(());
                        }
                    }

                }
                //找不到
                _ => {
                    Self::deposit_event(Event::CanNotFindGoods(products));  
                    return Ok(());
                }
            }
            //Self::deposit_event(Event::Addproduct(who.clone(), products));
            //Ok(())
        }

        //给自己账号删除
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn removeproduct(origin: OriginFor<T>, products : u32) -> DispatchResult {

            let who = ensure_signed(origin)?;

            //用户是否存在
            let usertemp = <WhatUserBuy<T>>::try_get(who.clone());
            match usertemp {
               
                Ok(mut user) => {
                    //用户存在，判断用户是都有这个商品
                    for mut i in &mut user.products {
                        //有这个商品
                        if i.productid == products {

                            if i.productnums == 0 {
                                Self::deposit_event(Event::CanNotFindGoods(products));
                                Self::deposit_event(Event::Line290());
                                return Ok(());
                            } else {
                                let p = <ProductPrice<T>>::get(products);
                                match p {

                                    Some(pricetemp) => {
                                        user.totalprice -= pricetemp;
                                    }
                                    _ => {
                                    //肯定会存在 所以这里不做任何事
                                    //TODO
                                    //有更好的语法 if let
                                    }
                                
                            }

                            i.productnums = i.productnums - 1;
                            Self::deposit_event(Event::Removeproduct(who.clone(), products));
                            Self::deposit_event(Event::Line309());
                            return Ok(());
                            }

                            
                        }
                    }

                    //商品不存在 不能减
                    Self::deposit_event(Event::CanNotFindGoods(products));
                    Self::deposit_event(Event::Line315());
                    return Ok(());
                }
                _ => {
                    Self::deposit_event(Event::Nothisuser(who.clone()));
                    return Ok(());
                }
            }
           
        }






		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.name.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.

					<Something<T>>::put(Order::new(new));
					Ok(())
				},
			}
		}
	}
}
