#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
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
	pub type Something<T> = StorageValue<_, u32>;

	// gzh add 
    //购物车商品数量,不同id，对应不同的购物车
    #[pallet::storage]
    #[pallet::getter(fn totalnums)]
    pub type TotalNums<T : Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32>;

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

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}



		//gzh add 
		//给自己账号添加商品
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn addproduct(origin: OriginFor<T>, products : u32, price : u32) -> DispatchResult {

            let who = ensure_signed(origin)?;

            //1.判断商品是否存在，存在->num + 1
            //2.该用户总商品数量+1
            //3.该用户总开销 + 该商品的价格

            let temp = <Products<T>>::try_get(who.clone(), products);
            match temp {
                Ok(t) => {
                    <Products<T>>::remove(who.clone(), products);
                    <Products<T>>::insert(who.clone(), products, t + 1);
                }
                _ => {
                    <Products<T>>::insert(who.clone(), products, 1);
                }
            }       

            //增加who 商品总数量
            let temp = <TotalNums<T>>::try_get(who.clone());
            match temp {
                Ok(t) => {
                    <TotalNums<T>>::remove(who.clone());
                    <TotalNums<T>>::insert(who.clone(), t + 1);
                }
                _ => {
                    <TotalNums<T>>::insert(who.clone(), 1);
                }
            }

            //增加who 总金额
            let temp = <TotalPrice<T>>::try_get(who.clone());
            match temp {
                Ok(t) => {
                    <TotalPrice<T>>::remove(who.clone());
                    <TotalPrice<T>>::insert(who.clone(), t + price);
                }
                _ => {
                    //用户不存在 直接insert
                    <TotalPrice<T>>::insert(who.clone(), price);
                }
            }

            Self::deposit_event(Event::Addproduct(who.clone(), products));
            Ok(())
        }

        //给自己账号删除
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn removeproduct(origin: OriginFor<T>, products : u32) -> DispatchResult {

            let who = ensure_signed(origin)?;

            //1.判断商品是否存在
            //2.商品存在 数量-1， 数量为0直接删除
            //3.商品不存在，无视操作

            let temp = <Products<T>>::try_get(who.clone(), products);
            match temp {
                Ok(num) => {
                    //user have this product
                    // num == 1 remove 
                    if num == 1 {
                        <Products<T>>::remove(who.clone(), products);
                    } else {
                        <Products<T>>::remove(who.clone(), products);
                        <Products<T>>::insert(who.clone(), products, num - 1);
                    }

                    //  减少 用户总数量
                    //  已经存在于products map 里面 直接把总数量减一即可
                    let products_nums = <TotalNums<T>>::get(who.clone());
                    match products_nums {
                        Some(nums) => {
                            <TotalNums<T>>::remove(who.clone());
                            <TotalNums<T>>::insert(who.clone(), nums - 1);
                        }
                        _ => {
                            //不做任何事
                        }
                    }


                    // 减少用户总开销
                    // TODO
                    // 增加一个map 存储  商品id ： 价钱
                }
                _ => {
                    //该user 没有这个商品 无视操作
                }
            }


            //发送 删除商品事件
            Self::deposit_event(Event::Removeproduct(who.clone(), products));

            Ok(())
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
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}
